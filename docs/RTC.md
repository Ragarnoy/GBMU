# Real Time Clock

The **Real Time Clock** (refered as RTC in this document) of certain Cartridge
allow a Game to kept track of time even when the Gameboy is off.

To do that the cartridge power an hardware RTC with its on-cartridge battery.

## RTC registers

The RTC have the following registers.
All of the register are contained in a `u8`.

The RTC regs all to keep track of the time up to `512 days 23 hours 59 minutes 59 seconds`

| Name    | Description                               | Range Value |
| ------- | ----------------------------------------- | ----------- |
| Seconds |                                           | `00-59`     |
| Minutes |                                           | `00-59`     |
| Hours   |                                           | `00-23`     |
| Days    |                                           | `000-256`   |
| Control | see [control regs](#RTC_Control_Register) |             |

### RTC Control Register

The `RTC Control Regs` is a bitfields with the following field

| Index | Name              | Description                                 |
| ----- | ----------------- | ------------------------------------------- |
| **0** | Upper Day Counter | The most significant bit of the day counter |
| **6** | Halt              | halt the clock (0=Active, 1=Inactive)       |
| **7** | Day Counter Carry | Set when the day counter overflow           |

## Implementation

Each RTC implementation are per game,
since it's an harware module that is on cartridge that need it,
and not on the GameBoy.

For the implementation, we have to thing that is distincte:

- Simulating the RTC while the game is running (the software emulator is running)
- Simulating the RTC while the game is off (the software emulator is not running or running another game)

### Implementation while the emulator is not running

This section discuss how we can emulate the RTC while the emulator is not running or running another game.

#### The naive implementation when the emulator is not running

The naive implementation when the emulator is not running would be:

1. Before quitting:
  - Save the current timestamp, along side the RTC register
2. When loading the save:
  - Get diff between the current timestamp and the save one: this will get the elapsed time while the *Gameboy* was off
  - This *elapsed time* will offset the saved RTC registers

### Implementation while the emulator is running

This section discuss how we can emulate the RTC while the emulator is running the game that is need it.

#### The naive implementation when the emulator

The naive implementation when the emulator is running would be:

1. Have a `NaiveDate` that represent the `RTC regs`
2. When the clock is ticking (`halt` is disabled):
  1. Start an **Instant** (a non decreasing clock that is usefull to calculate a Duration).
  2. When a RTC register is **READ**:
    1. Calculate the elapsed time between the previous `Instant` and the current one
    2. Use the elapsed time to update the `NaiveDate`
    3. Retrieve the register the user want to read
    4. Reset the initial Instant
  3. When a RTC register is **Writted**:
    1. Update the register in the `NaiveDate`
3. When the clock is not ticking (`halt` is enabled)
  1. When a register is **READ**: return the expected register value
  2. When a register is **Writted**: update the register with the new value

## Sources

- [The Clock Counter Registers](https://gbdev.io/pandocs/MBC3.html#the-clock-counter-registers)
- [The Day Counter](https://gbdev.io/pandocs/MBC3.html#the-day-counter)
