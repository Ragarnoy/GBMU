# GameBoy Address Bus

The Gameboy use a 16-bit address bus, so the gameboy is limited to address in the range `0-0xffff` (or `0-65535`) that 64KiB of mapped data

- [GameBoy Address Bus](#gameboy-address-bus)
  - [How the Adress Bus is architectured](#how-the-adress-bus-is-architectured)
  - [Boot ROM](#boot-rom)
  - [Sources](#sources)

## How the Adress Bus is architectured

The Adress bus is mapped like the following:

| Type         | Lower Bound | Upper Bound | Description                            |
| ------------ | :---------: | :---------: | -------------------------------------- |
| ROM          |  `0x0000`   |  `0x7FFF`   | ROM data area (readonly)               |
| Video RAM    |  `0x8000`   |  `0x9FFF`   |                                        |
| External RAM |  `0xA000`   |  `0xBFFF`   | Optional RAM provided by the Cartridge |
| RAM          |  `0xC000`   |  `0xDFFF`   | Internal RAM provided by the Gameboy   |
| ERAM         |  `0xE000`   |  `0xFDFF`   | Echo RAM, mirror of `[0xC000-0xDDFF]`  |
| OAM RAM      |  `0xFE00`   |  `0xFE9F`   | Sprite Attribute Table                 |
|              |  `0xFEA0`   |  `0xFEFF`   | Not Usable                             |
| I/O          |  `0xFF00`   |  `0xFF7F`   | I/O register                           |
| HRAM         |  `0xFF80`   |  `0xFFFE`   | High RAM (mean faster acess for GB)    |
| IE           |  `0xFFFF`   |  `0xFFFF`   | Interrupt Enable Register              |

## Boot ROM

Boot ROM or BIOS is code that is runned by the Gameboy before the ROM. \
The BIOS is first map to `0x0000-0x00ff`. \
After success this area is taken over by the ROM of the cartridge.

## Sources

- [Memory Map](https://gbdev.io/pandocs/Memory_Map.html)
