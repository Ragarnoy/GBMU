# Cpu Timing

the cpu timing is done using a micro code that perform micro action.
Each `action` take some cycle to execute.

## Micro code

| Action   | Description          | Timing (M-Cycle) |
| -------- | -------------------- | ---------------- |
| fetch    | fetch the opc id     | 1                |
| read     | read the next byte   | 1                |
| internal | internal computation | 0/1              |
| write    | write data           | 1                |

PS: `internal` action may take no time on branch decision (example: jump on condition) when the decision is not true

## Examples

### LD B,u8

1. fetch: read the opcode id
2. read: write the read byte to B

### JP NZ,u16

1. fetch
2. read u16::lower
3. read u16::upper
4. ? jump to u16 when the flag zero is **not** set

## Sources

- [The Game Boy opcode table](https://izik1.github.io/gbops/index.html)
