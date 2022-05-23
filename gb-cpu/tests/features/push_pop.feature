Feature: Test stack opcode
  Scenario: Test Push opcode
    Given the bytes D5 at the position PC
    And the register DE set to the value BEEF
    And the register SP set to the value 42
    When the cpu has ticked 4 times
    Then the opcode was PushDe
    And the cpu has no action left
    And the register SP is set to 40
    And the values written at 40 are EF, BE

  Scenario: Test Pop opcode
    Given the bytes E1, 00 at the position PC
    And the register SP set to the value 42
    And the bytes EF, BE at the position SP
    And the register SP set to the value 42
    When the cpu has ticked 3 times
    Then the opcode was PopHl
    When the cpu has ticked 1 time
    Then the opcode was Nop
    And the cpu has no action left
    And the register SP is set to 44
    And the register HL is set to BEEF
