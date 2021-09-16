# Cpu Timing

the cpu timing is done using a micro code that perform micro action.
Each `action` take some cycle to execute.

| Action   | Description          | Timing (M-Cycle) |
| -------- | -------------------- | ---------------- |
| fetch    | fetch the opc id     | 1                |
| read     | read the next byte   | 1                |
| internal | internal computation | 0/1              |
| write    | write data           | 1                |

## Sources

- [The Game Boy opcode table](https://izik1.github.io/gbops/index.html)
