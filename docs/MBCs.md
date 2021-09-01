# Memory Bank Controllers

The Gameboy has a limited spaced with his 16-bit address bus. \
To overcome this limitation many Games are using Memory Bank Controllers (MBC). \
A MBC allow to expand the available address space by bank switching.

---

- [Memory Bank Controllers](#memory-bank-controllers)
  - [No MBC](#no-mbc)
  - [MBC1](#mbc1)
    - [Architecture of MBC1](#architecture-of-mbc1)
      - [Enable RAM](#enable-ram)
      - [Change ROM Bank Number](#change-rom-bank-number)
        - [Warning on special value](#warning-on-special-value)
      - [Special 2-bit register](#special-2-bit-register)
      - [Change Banking Mode](#change-banking-mode)
        - [Rom Mode](#rom-mode)
        - [Ram Mode](#ram-mode)
  - [MBC2](#mbc2)
    - [Architecture of MBC2](#architecture-of-mbc2)
    - [Registers of MBC2](#registers-of-mbc2)
      - [Enable the RAM of MBC2](#enable-the-ram-of-mbc2)
      - [Select the ROM bank of MBC2](#select-the-rom-bank-of-mbc2)
  - [MBC3](#mbc3)
    - [Architecture of MBC3](#architecture-of-mbc3)
    - [Registers of MBC3](#registers-of-mbc3)
      - [Real Time Clock register](#real-time-clock-register)
      - [RAM and Timer enabling](#ram-and-timer-enabling)
      - [Select the Rom Bank Number of MBC3](#select-the-rom-bank-number-of-mbc3)
      - [Ram Bank Number or RTC register Selection](#ram-bank-number-or-rtc-register-selection)
      - [Latch Clock Data](#latch-clock-data)
  - [MBC5](#mbc5)
    - [Architecture of MBC5](#architecture-of-mbc5)
    - [Registers of MBC5](#registers-of-mbc5)
      - [Ram enabling](#ram-enabling)
      - [Selecting the Rom Bank of MBC5](#selecting-the-rom-bank-of-mbc5)
    - [Selecting the RAM bank of MBC5](#selecting-the-ram-bank-of-mbc5)
  - [Sources](#sources)

## No MBC

Cartridge with no `MBC` or `ROM only` (i.e.: with a rom size `<32Kib` or `<0x8000`)
are directly load into the `Rom Address Space`. \
Optionally up to 8KiB of RAM could be connected at

## MBC1

`MCB1` cartridges are limited to 2MByte ROM and/or 32 KiB RAM

### Architecture of MBC1

| Name       | Lower Bound | Upper Bound | Description                                    | Mode         |
| ---------- | :---------: | :---------: | ---------------------------------------------- | ------------ |
| ROM Bank 0 |  `0x0000`   |  `0x3FFF`   | contain the first 16 KiB of Cartridge ROM      | Read Only    |
| ROM Bank n |  `0x4000`   |  `0x7FFF`   | Contain the 16 KiB of the Cartridge ROM Bank n | Read Only    |
| RAM Bank n |  `0xA000`   |  `0xBFFF`   | Contain the Cartridge RAM Bank n               | Read / Write |

#### Enable RAM

Before reading or writting to the external RAM, the game must enable it before (to prevent loose of data on expected shutdown).

To enable the RAM the game **MUST** write `0x0A` in the range of `0x0000-0x1FFF`, to disable he **MUST** write `0x00` in the same range

#### Change ROM Bank Number

To change the selected ROM Bank the game must write a byte in the range of `0x2000-0x3FFF`.

The bank number can be in the range `0x01-0x1F` so only the first 5-bits are taken in account.

> The game write `0xE1` into `0x2000`, `0xE1 = 0b1110_0001`. \
> We kept the first 5 bits : `n = (0xE1 & 0x1F) = 0x1`, so the bank **1** is selected

when the game need to use a bank number `> 0x1f` see [banking mode](#change-banking-mode)

##### Warning on special value

When `0` is written, the MBC tranaltes that value to `1`

> A bank the first 5 bit set to 0 cannot be selected

| Value  | Mapped |
| :----: | :----: |
| `0x00` | `0x01` |
| `0x20` | `0x21` |
| `0x40` | `0x41` |
| `0x60` | `0x61` |

#### Special 2-bit register

When writing in the range of `0x4000-0x5FFF` set a 2-bit register that will be used for the [banking mode](#change-banking-mode)

#### Change Banking Mode

`MBC1` have 2 banking modes: `ROM` and `RAM`.
These modes determine how the [secondary 2-bit register](#special-2-bit-register) is used.

You can change the mode by writing in the range of `0x6000-0x7FFF` the following value:

- `0x00` to enable `ROM` mode
- `0x01` to enable `RAM` mode

##### Rom Mode

When `ROM` mode is enable, the ROM bank n is the concatenation of the *special 2-bit register* and the *5-bit rom number register*

##### Ram Mode

When `RAM` mode is enable, the RAM bank number is the value of the *special 2-bit register*

## MBC2

`MBC2` cartridges are limited to 256 KiB of ROM and 512x4 *bits* (not byte) of RAM

### Architecture of MBC2

| Name       | Lower Bound | Upper Bound | Description                                 | Mode         |
| ---------- | :---------: | :---------: | ------------------------------------------- | ------------ |
| ROM Bank 0 |  `0x0000`   |  `0x3FFF`   | Contains the first 16 KiB of the ROM        | Read Only    |
| ROM Bank n |  `0x40000`  |  `0x7FFF`   | Contains the 16 KiB of the Cartridge Bank n | Read Only    |
| Ram        |  `0xA000`   |  `0xA1FF`   | Contains the RAM builtin the controller     | Read / Write |
| Echo Ram   |  `0xA200`   |  `0xBFFF`   | Echoe of the RAM `0xA000-0xA1FF`            | Read / Write |

> - For the `ROM Bank n`: The controller come with a builtin RAM of 512x4 bits so only the bottom 4bits **SHOULD** be used of the Byte
> - For the `ERAM`: only the bottom 9 bits of the address is used so it wrappe around in the range `0xA200-0xBFFF`

### Registers of MBC2

`MBC2` have registers to:

- [enable the RAM](#enable-the-ram-of-mbc2)
- [select the ROM](#select-the-rom-bank-of-mbc2)

Both registers can be modified by writing in the range `0x0000-0x3FFF` while respecting a specific condition for each register (mostly when the bit 8 in `on/off` in the address)

#### Enable the RAM of MBC2

When the address have his 8th bit off (`(addr & 0x100) == 0`), the value written can enable/disable the ram

**BY Default** the RAM is disabled.

If the value written is `0x0A` then it enable the RAM otherwise it disable the RAM.

Example of valid address:

- `0x0000-0x00FF`
- `0x0200-0x02FF`
- `0x0400-0x04FF`
- `0x3E00-0x3EFF`

#### Select the ROM bank of MBC2

When the address have his 8th bit on (`(addr & 0x100) == 1`), the value that is written control the selected ROM Bank at `0x4000-0x7FFF`

**BY Default** The ROM bank is 1

Only the lower 4 bits are taken into account (max rom are 16). when the value is `0` default to `1`

## MBC3

`MBC3` cartridges are limited to 2MiB of ROM and 32KiB of RAM

### Architecture of MBC3

| Name            | Lower Bound | Upper Bound | Description                                        | Mode         |
| --------------- | :---------: | :---------: | -------------------------------------------------- | ------------ |
| ROM Bank 0      |  `0x0000`   |  `0x3FFF`   | Contains the first 16 KiB of the ROM               | Read Only    |
| ROM Bank n      |  `0x40000`  |  `0x7FFF`   | Contains the 16 KiB of the Cartridge Bank n        | Read Only    |
| Ram Bank or RTC |  `0xA000`   |  `0xBFFF`   | Contains the RAM bank n or the register of the RTC | Read / Write |

### Registers of MBC3

`MBC3` have the following register:

- Real Time Clock register
- [RAM/Timer enabling](#ram-and-timer-enabling)
- [ROM Number](#select-the-rom-bank-number-of-mbc3)
- [RAM/RTC mode](#ram-bank-number-or-rtc-register-selection)
- [Latch Clock Data](#latch-clock-data)

#### Real Time Clock register

The `MBC3` has an internal clock that can be read by [latching the clock data](#latch-clock-data) and [Selecting the RTC mode](#ram-bank-number-or-rtc-register-selection)

The RTC have the following register with their corresponding index value for selection

| Name                  | Range   | Id     | Description                                                                                |
| --------------------- | ------- | ------ | ------------------------------------------------------------------------------------------ |
| Seconds               | `0-59`  | `0x08` |                                                                                            |
| Minutes               | `0-59`  | `0x09` |                                                                                            |
| Hours                 | `0-23`  | `0x0A` |                                                                                            |
| Day Couter (DC) Lower | `0-255` | `0x0B` | The lower 8 bits of the Day Counter                                                        |
| DC Upper, Halt, Carry |         | `0x0C` | bit 0 => 9th bit of the Day Counter,<br> bit 6 => Halt,<br> bit 7 => Day Counter Carry Bit |

#### RAM and Timer enabling

Writing a value of `0x0A` into the area `0x0000-0x1FFF` will enable the RAM / TIMER registers mapping into `0xA000-0xBFFF`

#### Select the Rom Bank Number of MBC3

When writting into the area `0x2000-0x3FFF`, the 7th first bits of the value will indicated the ROM Bank Number.

When writting the value `0` it'll default to bank number `1`.

Example: `0xC5 (0b1100_0101)` => `0x45 (0b0100_0101)`

#### Ram Bank Number or RTC register Selection

Writting specific value into the area `0x4000-0x5FFF` allow the select:

- The RAM Bank Number when the value is `0x00-0x03` and map it to `0xA000-0xBFFF`
- The RTC register when the value is `0x08-0x0C` and map it to `0xA000-0xBFFF`
  > typically the adress `0xA000` is used the map the register value

#### Latch Clock Data

When writing the sequence `0x00 -> 0x01` in the area `0x6000-0x7FFF` will update the RTC register from the internal clock of the cartridge.

This set the register to represente the current time of the internal clock, a syncronisation between the register and the clock

## MBC5

`MBC5` are limited to 8MiB of ROM and 128 KiB of RAM

### Architecture of MBC5

| Name       | Lower Bound | Upper Bound | Description                                 | Mode         |
| ---------- | :---------: | :---------: | ------------------------------------------- | ------------ |
| ROM Bank 0 |  `0x0000`   |  `0x3FFF`   | Contains the first 16 KiB of the ROM        | Read Only    |
| ROM Bank n |  `0x40000`  |  `0x7FFF`   | Contains the 16 KiB of the Cartridge Bank n | Read Only    |
| Ram Bank n |  `0xA000`   |  `0xBFFF`   | Contains the Ram Bank n of the Cartridge    | Read / Write |

### Registers of MBC5

`MBC5` have the following register:

- [Ram enabling](#ram-enabling)
- [Rom Bank number](#selecting-the-rom-bank-of-mbc5)
- [Ram Bank number](#selecting-the-ram-bank-of-mbc5)

#### Ram enabling

To enable the ram for `read/write` operation, you need to write the value `0x0A` into the area of `0x0000-0x1FFF`.
You can disable the ram by writing `0x00`

> Apperently only the 4th least significant bits are use the enable or not the RAM, so writing `0x1A` will enable it

#### Selecting the Rom Bank of MBC5

To selected the Rom bank number, you've 2 area were to write for selecting the ROM bank number since his value can be contain in a 9-bits wide number (max value is `0x1FF`)

Writting to the area `0x2000-0x2FFF` allow to set the first 8th bits of the bank number
Writting to the area `0x3000-0x3FFF` allow to set the 9th bits of the bank number

> As opposed as the previous MBCs setting the value `0` will not default to `1` but effectively select the ROM bank `0`

### Selecting the RAM bank of MBC5

To select the RAM bank number, write the value from the range `0x00-0x0F` into the area `0x4000-0x5FFF`

## Sources

- [MBCs](https://gbdev.io/pandocs/MBCs.html)
  - [No MBC](https://gbdev.io/pandocs/nombc.html)
  - [MBC1](https://gbdev.io/pandocs/MBC1.html)
  - [MBC2](https://gbdev.io/pandocs/MBC2.html)
  - [MBC3](https://gbdev.io/pandocs/MBC3.html)
- [How to switch bank](https://b13rg.github.io/Gameboy-Bank-Switching/)
