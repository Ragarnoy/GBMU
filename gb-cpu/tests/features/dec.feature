Feature: Test Dec opcodes

  Scenario Outline: test dec on generic values
    Given the bytes <Bytes> at the position PC
    And the u8 register <Reg> set to the value <Init>
    When the cpu has ticked 1 times
    Then the opcode was Dec<Reg>
    When the cpu has ticked 1 times
    Then the u8 register <Reg> is set to <End>

    Examples:
      | Reg | Bytes | Init | End |
      | B   | 05    | 42   | 41  |
      # test overflowing value
      | A   | 3D    | 0    | FF  |

  Scenario: test dec setting zero flag
    Given the bytes 05 at the position PC
    And the u8 register B set to the value 1
    When the cpu has ticked 1 times
    Then the opcode was DecB
    When the cpu has ticked 1 times
    Then the u8 register B is set to 0
    And the flag zero is set
