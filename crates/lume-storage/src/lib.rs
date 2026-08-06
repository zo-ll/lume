use std::{
    collections::BTreeMap,
    path::Path,
    sync::{Mutex, MutexGuard},
};

use lume_protocol::{encoded_observation, observation_digest, v1};
use prost::Message;
use rusqlite::{Connection, OptionalExtension, params};
use thiserror::Error;

#[derive(Debug)]
pub struct HistoryStore {
    connection: Mutex<Connection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AppendOutcome {
    Inserted { highest_contiguous_sequence: u64 },
    Duplicate { highest_contiguous_sequence: u64 },
    Conflict { highest_contiguous_sequence: u64 },
}

impl AppendOutcome {
    pub fn highest_contiguous_sequence(&self) -> u64 {
        match self {
            Self::Inserted {
                highest_contiguous_sequence,
            }
            | Self::Duplicate {
                highest_contiguous_sequence,
            }
            | Self::Conflict {
                highest_contiguous_sequence,
            } => *highest_contiguous_sequence,
        }
    }
}

#[derive(Debug, Error)]
pub enum StoreError {
    #[error("history database lock is poisoned")]
    Poisoned,
    #[error("producer sequence must start at one")]
    InvalidSequence,
    #[error("producer sequence exceeds the Local History integer range")]
    SequenceOutOfRange,
    #[error("runtime, producer, and evidence identities must not be empty")]
    MissingIdentity,
    #[error(transparent)]
    Sql(#[from] rusqlite::Error),
    #[error(transparent)]
    Decode(#[from] prost::DecodeError),
}

impl HistoryStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let connection = Connection::open(path)?;
        connection.pragma_update(None, "journal_mode", "WAL")?;
        connection.pragma_update(None, "foreign_keys", true)?;
        connection.execute_batch(
            "
            CREATE TABLE IF NOT EXISTS observations (
                runtime_id TEXT NOT NULL,
                producer_id TEXT NOT NULL,
                producer_sequence INTEGER NOT NULL,
                evidence_id TEXT NOT NULL,
                content_hash BLOB NOT NULL,
                envelope BLOB NOT NULL,
                received_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP,
                PRIMARY KEY (runtime_id, producer_id, producer_sequence),
                UNIQUE (runtime_id, evidence_id)
            );
            CREATE TABLE IF NOT EXISTS quarantined_observations (
                id INTEGER PRIMARY KEY,
                runtime_id TEXT NOT NULL,
                producer_id TEXT NOT NULL,
                producer_sequence INTEGER NOT NULL,
                evidence_id TEXT NOT NULL,
                content_hash BLOB NOT NULL,
                envelope BLOB NOT NULL,
                reason TEXT NOT NULL,
                received_at TEXT NOT NULL DEFAULT CURRENT_TIMESTAMP
            );
            ",
        )?;
        Ok(Self {
            connection: Mutex::new(connection),
        })
    }

    pub fn open_in_memory() -> Result<Self, StoreError> {
        Self::open(":memory:")
    }

    pub fn append_observation(
        &self,
        runtime_id: &str,
        observation: &v1::Observation,
    ) -> Result<AppendOutcome, StoreError> {
        if runtime_id.is_empty()
            || observation.producer_id.is_empty()
            || observation.evidence_id.is_empty()
        {
            return Err(StoreError::MissingIdentity);
        }
        if observation.producer_sequence == 0 {
            return Err(StoreError::InvalidSequence);
        }
        let producer_sequence = i64::try_from(observation.producer_sequence)
            .map_err(|_| StoreError::SequenceOutOfRange)?;

        let envelope = encoded_observation(observation);
        let digest = observation_digest(observation);
        let mut connection = self.connection()?;
        let transaction = connection.transaction()?;
        let existing_hash: Option<Vec<u8>> = transaction
            .query_row(
                "SELECT content_hash FROM observations
                 WHERE runtime_id = ?1 AND (
                    (producer_id = ?2 AND producer_sequence = ?3) OR evidence_id = ?4
                 ) LIMIT 1",
                params![
                    runtime_id,
                    observation.producer_id,
                    producer_sequence,
                    observation.evidence_id
                ],
                |row| row.get(0),
            )
            .optional()?;

        let kind = if let Some(existing_hash) = existing_hash {
            if existing_hash == digest {
                AppendKind::Duplicate
            } else {
                transaction.execute(
                    "INSERT INTO quarantined_observations (
                        runtime_id, producer_id, producer_sequence, evidence_id,
                        content_hash, envelope, reason
                     ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 'identity_conflict')",
                    params![
                        runtime_id,
                        observation.producer_id,
                        producer_sequence,
                        observation.evidence_id,
                        digest.as_slice(),
                        envelope
                    ],
                )?;
                AppendKind::Conflict
            }
        } else {
            transaction.execute(
                "INSERT INTO observations (
                    runtime_id, producer_id, producer_sequence, evidence_id,
                    content_hash, envelope
                 ) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                params![
                    runtime_id,
                    observation.producer_id,
                    producer_sequence,
                    observation.evidence_id,
                    digest.as_slice(),
                    envelope
                ],
            )?;
            AppendKind::Inserted
        };

        let highest =
            highest_contiguous_in(&transaction, runtime_id, observation.producer_id.as_str())?;
        transaction.commit()?;
        Ok(match kind {
            AppendKind::Inserted => AppendOutcome::Inserted {
                highest_contiguous_sequence: highest,
            },
            AppendKind::Duplicate => AppendOutcome::Duplicate {
                highest_contiguous_sequence: highest,
            },
            AppendKind::Conflict => AppendOutcome::Conflict {
                highest_contiguous_sequence: highest,
            },
        })
    }

    pub fn acknowledged_positions(
        &self,
        runtime_id: &str,
    ) -> Result<BTreeMap<String, u64>, StoreError> {
        let connection = self.connection()?;
        let mut producers = connection
            .prepare("SELECT DISTINCT producer_id FROM observations WHERE runtime_id = ?1")?;
        let ids: Vec<String> = producers
            .query_map([runtime_id], |row| row.get(0))?
            .collect::<Result<_, _>>()?;
        ids.into_iter()
            .map(|producer| {
                let highest = highest_contiguous_in(&connection, runtime_id, &producer)?;
                Ok((producer, highest))
            })
            .collect()
    }

    pub fn observation_count(&self) -> Result<u64, StoreError> {
        let connection = self.connection()?;
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM observations", [], |row| row.get(0))
            .map_err(StoreError::from)?;
        u64::try_from(count).map_err(|_| StoreError::SequenceOutOfRange)
    }

    pub fn quarantine_count(&self) -> Result<u64, StoreError> {
        let connection = self.connection()?;
        let count: i64 = connection
            .query_row("SELECT COUNT(*) FROM quarantined_observations", [], |row| {
                row.get(0)
            })
            .map_err(StoreError::from)?;
        u64::try_from(count).map_err(|_| StoreError::SequenceOutOfRange)
    }

    pub fn observations(&self) -> Result<Vec<v1::Observation>, StoreError> {
        let connection = self.connection()?;
        let mut statement = connection.prepare(
            "SELECT envelope FROM observations
             ORDER BY runtime_id, producer_id, producer_sequence",
        )?;
        let bytes: Vec<Vec<u8>> = statement
            .query_map([], |row| row.get(0))?
            .collect::<Result<_, _>>()?;
        bytes
            .into_iter()
            .map(|bytes| v1::Observation::decode(bytes.as_slice()).map_err(Into::into))
            .collect()
    }

    fn connection(&self) -> Result<MutexGuard<'_, Connection>, StoreError> {
        self.connection.lock().map_err(|_| StoreError::Poisoned)
    }
}

#[derive(Clone, Copy, Debug)]
enum AppendKind {
    Inserted,
    Duplicate,
    Conflict,
}

fn highest_contiguous_in(
    connection: &Connection,
    runtime_id: &str,
    producer_id: &str,
) -> Result<u64, rusqlite::Error> {
    let mut statement = connection.prepare(
        "SELECT producer_sequence FROM observations
         WHERE runtime_id = ?1 AND producer_id = ?2
         ORDER BY producer_sequence",
    )?;
    let positions =
        statement.query_map(params![runtime_id, producer_id], |row| row.get::<_, i64>(0))?;
    let mut highest = 0;
    for position in positions {
        let position =
            u64::try_from(position?).expect("producer sequences are stored non-negative");
        if position == highest + 1 {
            highest = position;
        } else if position > highest + 1 {
            break;
        }
    }
    Ok(highest)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lume_protocol::v1::{TraceStarted, observation::Evidence};
    use tempfile::tempdir;

    fn observation(sequence: u64, evidence_id: &str, objective: &str) -> v1::Observation {
        v1::Observation {
            producer_id: "producer".into(),
            producer_sequence: sequence,
            evidence_id: evidence_id.into(),
            evidence: Some(Evidence::TraceStarted(TraceStarted {
                trace_id: "trace".into(),
                agent_id: "agent".into(),
                objective: objective.into(),
            })),
        }
    }

    #[test]
    fn acknowledgement_advances_only_across_durable_contiguous_positions() {
        let store = HistoryStore::open_in_memory().unwrap();
        let third = store
            .append_observation("runtime", &observation(3, "third", "test"))
            .unwrap();
        assert_eq!(third.highest_contiguous_sequence(), 0);
        store
            .append_observation("runtime", &observation(1, "first", "test"))
            .unwrap();
        let second = store
            .append_observation("runtime", &observation(2, "second", "test"))
            .unwrap();
        assert_eq!(second.highest_contiguous_sequence(), 3);
    }

    #[test]
    fn identical_replay_is_idempotent_and_changed_replay_is_quarantined() {
        let store = HistoryStore::open_in_memory().unwrap();
        let original = observation(1, "evidence", "original");
        assert!(matches!(
            store.append_observation("runtime", &original).unwrap(),
            AppendOutcome::Inserted { .. }
        ));
        assert!(matches!(
            store.append_observation("runtime", &original).unwrap(),
            AppendOutcome::Duplicate { .. }
        ));
        assert!(matches!(
            store
                .append_observation("runtime", &observation(1, "evidence", "changed"))
                .unwrap(),
            AppendOutcome::Conflict { .. }
        ));
        assert_eq!(store.observation_count().unwrap(), 1);
        assert_eq!(store.quarantine_count().unwrap(), 1);
    }

    #[test]
    fn committed_observations_survive_restart() {
        let directory = tempdir().unwrap();
        let path = directory.path().join("history.sqlite");
        {
            let store = HistoryStore::open(&path).unwrap();
            store
                .append_observation("runtime", &observation(1, "evidence", "test"))
                .unwrap();
        }
        let reopened = HistoryStore::open(&path).unwrap();
        assert_eq!(reopened.observation_count().unwrap(), 1);
        assert_eq!(
            reopened.acknowledged_positions("runtime").unwrap()["producer"],
            1
        );
    }
}
