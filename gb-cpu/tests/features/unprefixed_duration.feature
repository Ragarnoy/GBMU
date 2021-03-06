Feature: Verify unprefixed opcodes duration
  Scenario Outline: Verify unprefixed opcodes duration
    Given the register SP set to the value BEEF
    And the register PC set to the value DEAD
    And the register HL set to the value F000
    And the bytes <Bytes>, DE, DE at the position PC
    Then the cpu has ticked <N> times for the current opcode <Name>

    Examples:
      | Name     | N | Bytes |
      | Nop      | 1 | 00    |
      | LdBC16   | 3 | 01    |
      | LdBCA    | 2 | 02    |
      | IncBC    | 2 | 03    |
      | IncB     | 1 | 04    |
      | DecB     | 1 | 05    |
      | LdB8     | 2 | 06    |
      | RlcA     | 1 | 07    |
      | Ld16SP   | 5 | 08    |
      | AddHLBC  | 2 | 09    |
      | LdABC    | 2 | 0A    |
      | DecBC    | 2 | 0B    |
      | IncC     | 1 | 0C    |
      | DecC     | 1 | 0D    |
      | LdC8     | 2 | 0E    |
      | RrcA     | 1 | 0F    |
      | Stop     | 1 | 10    |
      | LdDE16   | 3 | 11    |
      | LdDEA    | 2 | 12    |
      | IncDE    | 2 | 13    |
      | IncD     | 1 | 14    |
      | DecD     | 1 | 15    |
      | LdD8     | 2 | 16    |
      | Rla      | 1 | 17    |
      | Jr       | 3 | 18    |
      | AddHLDE  | 2 | 19    |
      | LdADE    | 2 | 1A    |
      | DecDE    | 2 | 1B    |
      | IncE     | 1 | 1C    |
      | DecE     | 1 | 1D    |
      | LdE8     | 2 | 1E    |
      | Rra      | 1 | 1F    |
      | LdHL16   | 3 | 21    |
      | LdiHLA   | 2 | 22    |
      | IncHL    | 2 | 23    |
      | IncH     | 1 | 24    |
      | DecH     | 1 | 25    |
      | LdH8     | 2 | 26    |
      | Daa      | 1 | 27    |
      | AddHLHL  | 2 | 29    |
      | LdiAHL   | 2 | 2A    |
      | DecHL    | 2 | 2B    |
      | IncL     | 1 | 2C    |
      | DecL     | 1 | 2D    |
      | LdL8     | 2 | 2E    |
      | Cpl      | 1 | 2F    |
      | LdSP16   | 3 | 31    |
      | LddHLA   | 2 | 32    |
      | IncSP    | 2 | 33    |
      | IncHLind | 3 | 34    |
      | DecHLind | 3 | 35    |
      | LdHL8    | 3 | 36    |
      | Scf      | 1 | 37    |
      | AddHLSP  | 2 | 39    |
      | LddAHL   | 2 | 3A    |
      | DecSP    | 2 | 3B    |
      | IncA     | 1 | 3C    |
      | DecA     | 1 | 3D    |
      | LdA8     | 2 | 3E    |
      | Ccf      | 1 | 3F    |
      | LdBB     | 1 | 40    |
      | LdBC     | 1 | 41    |
      | LdBD     | 1 | 42    |
      | LdBE     | 1 | 43    |
      | LdBH     | 1 | 44    |
      | LdBL     | 1 | 45    |
      | LdBHL    | 2 | 46    |
      | LdBA     | 1 | 47    |
      | LdCB     | 1 | 48    |
      | LdCC     | 1 | 49    |
      | LdCD     | 1 | 4A    |
      | LdCE     | 1 | 4B    |
      | LdCH     | 1 | 4C    |
      | LdCL     | 1 | 4D    |
      | LdCHL    | 2 | 4E    |
      | LdCA     | 1 | 4F    |
      | LdDB     | 1 | 50    |
      | LdDC     | 1 | 51    |
      | LdDD     | 1 | 52    |
      | LdDE     | 1 | 53    |
      | LdDH     | 1 | 54    |
      | LdDL     | 1 | 55    |
      | LdDHL    | 2 | 56    |
      | LdDA     | 1 | 57    |
      | LdEB     | 1 | 58    |
      | LdEC     | 1 | 59    |
      | LdED     | 1 | 5A    |
      | LdEE     | 1 | 5B    |
      | LdEH     | 1 | 5C    |
      | LdEL     | 1 | 5D    |
      | LdEHL    | 2 | 5E    |
      | LdEA     | 1 | 5F    |
      | LdHB     | 1 | 60    |
      | LdHC     | 1 | 61    |
      | LdHD     | 1 | 62    |
      | LdHE     | 1 | 63    |
      | LdHH     | 1 | 64    |
      | LdHL     | 1 | 65    |
      | LdHHL    | 2 | 66    |
      | LdHA     | 1 | 67    |
      | LdLB     | 1 | 68    |
      | LdLC     | 1 | 69    |
      | LdLD     | 1 | 6A    |
      | LdLE     | 1 | 6B    |
      | LdLH     | 1 | 6C    |
      | LdLL     | 1 | 6D    |
      | LdLHL    | 2 | 6E    |
      | LdLA     | 1 | 6F    |
      | LdHLB    | 2 | 70    |
      | LdHLC    | 2 | 71    |
      | LdHLD    | 2 | 72    |
      | LdHLE    | 2 | 73    |
      | LdHLH    | 2 | 74    |
      | LdHLL    | 2 | 75    |
      | Halt     | 1 | 76    |
      | LdHLA    | 2 | 77    |
      | LdAB     | 1 | 78    |
      | LdAC     | 1 | 79    |
      | LdAD     | 1 | 7A    |
      | LdAE     | 1 | 7B    |
      | LdAH     | 1 | 7C    |
      | LdAL     | 1 | 7D    |
      | LdAHL    | 2 | 7E    |
      | LdAA     | 1 | 7F    |
      | AddAB    | 1 | 80    |
      | AddAC    | 1 | 81    |
      | AddAD    | 1 | 82    |
      | AddAE    | 1 | 83    |
      | AddAH    | 1 | 84    |
      | AddAL    | 1 | 85    |
      | AddAHL   | 2 | 86    |
      | AddAA    | 1 | 87    |
      | AdcAB    | 1 | 88    |
      | AdcAC    | 1 | 89    |
      | AdcAD    | 1 | 8A    |
      | AdcAE    | 1 | 8B    |
      | AdcAH    | 1 | 8C    |
      | AdcAL    | 1 | 8D    |
      | AdcAHL   | 2 | 8E    |
      | AdcAA    | 1 | 8F    |
      | SubAB    | 1 | 90    |
      | SubAC    | 1 | 91    |
      | SubAD    | 1 | 92    |
      | SubAE    | 1 | 93    |
      | SubAH    | 1 | 94    |
      | SubAL    | 1 | 95    |
      | SubAHL   | 2 | 96    |
      | SubAA    | 1 | 97    |
      | SbcAB    | 1 | 98    |
      | SbcAC    | 1 | 99    |
      | SbcAD    | 1 | 9A    |
      | SbcAE    | 1 | 9B    |
      | SbcAH    | 1 | 9C    |
      | SbcAL    | 1 | 9D    |
      | SbcAHL   | 2 | 9E    |
      | SbcAA    | 1 | 9F    |
      | AndAB    | 1 | A0    |
      | AndAC    | 1 | A1    |
      | AndAD    | 1 | A2    |
      | AndAE    | 1 | A3    |
      | AndAH    | 1 | A4    |
      | AndAL    | 1 | A5    |
      | AndAHL   | 2 | A6    |
      | AndAA    | 1 | A7    |
      | XorAB    | 1 | A8    |
      | XorAC    | 1 | A9    |
      | XorAD    | 1 | AA    |
      | XorAE    | 1 | AB    |
      | XorAH    | 1 | AC    |
      | XorAL    | 1 | AD    |
      | XorAHL   | 2 | AE    |
      | XorAA    | 1 | AF    |
      | OrAB     | 1 | B0    |
      | OrAC     | 1 | B1    |
      | OrAD     | 1 | B2    |
      | OrAE     | 1 | B3    |
      | OrAH     | 1 | B4    |
      | OrAL     | 1 | B5    |
      | OrAHL    | 2 | B6    |
      | OrAA     | 1 | B7    |
      | CpAB     | 1 | B8    |
      | CpAC     | 1 | B9    |
      | CpAD     | 1 | BA    |
      | CpAE     | 1 | BB    |
      | CpAH     | 1 | BC    |
      | CpAL     | 1 | BD    |
      | CpAHL    | 2 | BE    |
      | CpAA     | 1 | BF    |
      | PopBc    | 3 | C1    |
      | Jp       | 4 | C3    |
      | PushBc   | 4 | C5    |
      | AddA8    | 2 | C6    |
      | Rst00    | 4 | C7    |
      | Call16   | 6 | CD    |
      | AdcA8    | 2 | CE    |
      | Rst08    | 4 | CF    |
      | PopDe    | 3 | D1    |
      | PushDe   | 4 | D5    |
      | SubA8    | 2 | D6    |
      | Rst10    | 4 | D7    |
      | SbcA8    | 2 | DE    |
      | Rst18    | 4 | DF    |
      | Ldh8A    | 3 | E0    |
      | PopHl    | 3 | E1    |
      | LdhCA    | 2 | E2    |
      | PushHl   | 4 | E5    |
      | AndA8    | 2 | E6    |
      | Rst20    | 4 | E7    |
      | AddSp8   | 4 | E8    |
      | JpHl     | 1 | E9    |
      | Ld16A    | 4 | EA    |
      | XorA8    | 2 | EE    |
      | Rst28    | 4 | EF    |
      | LdhA8    | 3 | F0    |
      | PopAf    | 3 | F1    |
      | LdhAC    | 2 | F2    |
      | Di       | 1 | F3    |
      | PushAf   | 4 | F5    |
      | OrA8     | 2 | F6    |
      | Rst30    | 4 | F7    |
      | LdhlSp8  | 3 | F8    |
      | LdSPHL   | 2 | F9    |
      | LdA16    | 4 | FA    |
      | Ei       | 1 | FB    |
      | CpA8     | 2 | FE    |
      | Rst38    | 4 | FF    |

  Scenario Outline: Verify unprefixed opcodes duration with condition
    Given the register SP set to the value BEEF
    And the register PC set to the value DEAD
    And the bytes <Bytes>, DE, DE at the position PC
    And the flag <Flag> is <Condition>
    Then the cpu has ticked <DBranch> times for the current opcode <Name>
    Given the cpu is reset
    And the register SP set to the value BEEF
    And the register PC set to the value DEAD
    And the bytes <Bytes> at the position PC
    And the flag <Flag> is <Condition>
    And the flag <Flag> is toggle
    Then the cpu has ticked <DNoBranch> times for the current opcode <Name>

    Examples:
      | Name     | Flag  | Condition | DNoBranch | DBranch | Bytes |
      | JrNz     | zero  | reset     | 2         | 3       | 20    |
      | JrZ      | zero  | set       | 2         | 3       | 28    |
      | JrNc     | carry | reset     | 2         | 3       | 30    |
      | JrC      | carry | set       | 2         | 3       | 38    |
      | JpNz     | zero  | reset     | 3         | 4       | C2    |
      | CallNz   | zero  | reset     | 3         | 6       | C4    |
      | JpZ      | zero  | set       | 3         | 4       | CA    |
      | CallZ16  | zero  | set       | 3         | 6       | CC    |
      | JpNc     | carry | reset     | 3         | 4       | D2    |
      | CallNc16 | carry | reset     | 3         | 6       | D4    |
      | JpC      | carry | set       | 3         | 4       | DA    |
      | CallC16  | carry | set       | 3         | 6       | DC    |

  Scenario Outline: Verify Ret opcodes duration with condition
    Given the following bytes
      | Address | Value |
      | BEEF    | DE    |
      | BEF0    | AD    |
    And the register SP set to the value BEEF
    And the bytes <Bytes> at the position PC
    And the flag <Flag> is <Condition>
    Then the cpu has ticked <DBranch> times for the current opcode <Name>
    Given the cpu is reset
    And the register SP set to the value BEEF
    And the bytes <Bytes> at the position PC
    And the flag <Flag> is <Condition>
    And the flag <Flag> is toggle
    Then the cpu has ticked <DNoBranch> times for the current opcode <Name>

    Examples:
      | Name  | Flag  | Condition | DNoBranch | DBranch | Bytes |
      | RetNz | zero  | reset     | 2         | 5       | C0    |
      | RetZ  | zero  | set       | 2         | 5       | C8    |
      | RetNc | carry | reset     | 2         | 5       | D0    |
      | RetC  | carry | set       | 2         | 5       | D8    |

  Scenario Outline: Verify Ret opcodes duration
    Given the following bytes
      | Address | Value |
      | BEEF    | DE    |
      | BEF0    | AD    |
    And the register SP set to the value BEEF
    And the bytes <Bytes> at the position PC
    Then the cpu has ticked <N> times for the current opcode <Name>

    Examples:
      | Name | N | Bytes |
      | Ret  | 4 | C9    |
      | Reti | 4 | D9    |


