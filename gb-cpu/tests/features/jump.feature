Feature: Test jump opcodes

  Scenario: Test Jump Opcode
    Given the bytes C3, AD, DE at the PC position
    When the cpu as ticked 4 times
    Then the opcode was Jp
    And the cpu as no action left
    And the PC register is set to DEAD
