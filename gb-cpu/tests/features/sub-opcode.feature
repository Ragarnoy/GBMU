Feature: Test opcodes that substrac 2 value
  Scenario: Test cp opcode
    Given the bytes FE, 34 at the position PC
    And the u8 register A set to the value 5
    When the cpu has ticked 1 times
    Then the opcode was CpA8
    When the cpu has ticked 2 times
    Then the flag substraction is set
    And the flag carry is set
    And the flag half carry is not set
