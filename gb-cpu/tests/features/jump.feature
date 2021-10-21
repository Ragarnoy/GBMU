Feature: Test jump opcodes

  Scenario: Test Jump Opcode
    Given the bytes C3, AD, DE at the position PC
    When the cpu has ticked 4 times
    Then the opcode was Jp
    And the cpu has no action left
    And the register PC is set to DEAD
