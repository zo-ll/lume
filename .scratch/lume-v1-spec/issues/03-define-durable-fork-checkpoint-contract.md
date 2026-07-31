# Define the durable Fork Checkpoint contract

Parent: [Specify Lume v1 causal debugging and trace forking](../map.md)
Type: `grilling`
Status: `open`
Blocked by: 01

## Question

What must an Instrumented Agent Runtime declare, persist, and later recover so Lume can identify a Fork Checkpoint, know whether it remains available, reconnect or relaunch the responsible runtime, and request restoration safely across runtime restarts?
