# Tile Data
Pixels values are stored in the vram, in `0x8000-0x97FF`. A pixel value is defined on 2 bits (so between 0 and 3). A couple of bytes defines 8 pixels values. The bits of the first byte are the least significant bits of the pixels values, and the bits of the second byte are the most significant bits of the pixels values. In both bytes, bit 7 represents the leftmost pixel, and bit 0 the rightmost.

## example
`0x3366` -> `00110011` `01100110`
| hexa   | bin        |
| ------ | ---------- |
| `0x33` | `00110011` |
| `0x66` | `01100110` |

Pixels values:
| Index | 1st bit | 2nd bit | Value |
| ----- | ------- | ------- | ----- |
| **0** | 0       | 0       | 0     |
| **1** | 0       | 1       | 2     |
| **2** | 1       | 1       | 3     |
| **3** | 1       | 0       | 1     |
| **4** | 0       | 0       | 0     |
| **5** | 0       | 1       | 2     |
| **6** | 1       | 1       | 3     |
| **7** | 1       | 0       | 1     |

 which gives the following line of pixels: 13201320

# Tile Map

Tile map values are 1 byte index of the tile.

## Map positions
The tile maps are stored in `0x9800-0x9BFF` and/or in `0x9C00-0x9FFF`.
The Background map adress is defined by the value of the register LCDC bit 3:
 - **bit is 0**: background map is in `0x9800-0x9BFF`.
 - **bit is 0**: background map is in `0x9C00-0x9FFF`.

The Window map adress is defined by the value of the register LCDC bit 6:
 - **bit is 0**: window map is in `0x9800-0x9BFF`.
 - **bit is 0**: window map is in `0x9C00-0x9FFF`.

## Tile sheet block
Depending on the value of the register LCDC bit 4, the tile index can point to a different block of the tile sheet:
 - **bit is 0**: index from the map start from `0x8000` in the tile sheet. The index is an unsigned byte so the tiles are stored in `0x8000-0x8FFF`.
 - **bit is 1**: index from the map start from `0x9000` in the tile sheet. The index is a signed byte so the tiles are stored in `0x8800-0x97FF`.

| block           | obj index | bg/win index if LCDC.4==0 | bg/win index if LCDC.4==1 |
| --------------- | --------- | ------------------------- | ------------------------- |
| `0x8000-0x87FF` | 0 - 127   | 0 - 127                   |                           |
| `0x8800-0x8FFF` | 128 - 255 | 128 - 255                 | -128 - -1                 |
| `0x9000-0x97FF` |           |                           | 0 - 127                   |

# OAM
The Object Attribute Table hold the data of 40 objects (sprites) in `0xFE00-0xFE9F`. An object can be a 8x8 or a 8x16 sprite. Each object is made of 4 bytes:
 - Y position
 - X position
 - Tile index
 - Attributes

At each scanline, the ppu selects up to 10 valid object according to their Y position.

## Y position
The Y position of the sprite + 16. A 8xH sprite is fully displayed on the screen with 168 - H > Y >= 16.

## X position
The X position of the sprite + 8. A sprite is fully displayed on the screen with 168 > X >= 8.

## Tile index
The sprite mode is defined by the bit 2 of LCDC:
- LCDC.2==0: 8x8 sprite mode, since a sprite is made of only one tile the tile index correspond to its index.
- LCDC.2==1: 8x16 sprite mode, a sprite is made of two adjacent tiles and the byte indicate the index of the first one. The first tile is the top of the sprite. Note that the hardware enforce the index to point at a pair value by ignoring the least significant bit of the byte.

## Attributes
- Bit 7: BG and Window over OBJ (0=No, 1=BG and Window colors 1-3 over the OBJ)
- Bit 6: Y flip (0=Normal, 1=Vertically mirrored)
- Bit 5: X flip (0=Normal, 1=Horizontally mirrored)
- Bit 4: Palette number  **Non CGB Mode Only** (0=OBP0, 1=OBP1)
- Bit 3: Tile VRAM-Bank  **CGB Mode Only**     (0=Bank 0, 1=Bank 1)
- Bit 2-0: Palette number  **CGB Mode Only**     (OBP0-7)

# Palettes

The monochrome palettes are stored in registers of one byte each (`FF47`, `FF48` and `FF49`).
A palette map a color index from the tile data to a color for the screen.

## Color
Each color is stored on 2 bits:
| Value | Color      |
| ----- | ---------- |
| 0     | white      |
| 1     | light gray |
| 2     | dark gray  |
| 3     | black      |

## BG palette data (BGP) - `FF47`
Bit 7-6 - Color for index 3
Bit 5-4 - Color for index 2
Bit 3-2 - Color for index 1
Bit 1-0 - Color for index 0

## Object palette 0 data (OBP0) - `FF48`
Bit 7-6 - Color for index 3
Bit 5-4 - Color for index 2
Bit 3-2 - Color for index 1
Bit 1-0 - ignored (index 0 is transparent for objects)

### Object palette 1 data (OBP1) - `FF49`
Bit 7-6 - Color for index 3
Bit 5-4 - Color for index 2
Bit 3-2 - Color for index 1
Bit 1-0 - ignored (index 0 is transparent for objects)
