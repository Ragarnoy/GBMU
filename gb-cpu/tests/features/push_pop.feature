Feature: Test stack opcode
  Scenario: Test Push opcode
    Given the bytes D5 at the position PC
    And the register DE set at the value BEEF
    And the register SP set to the value 42
    When the cpu has ticked 4 times
    Then the opcode was PushDE
    And the cpu has no action left
    And the u16 value written at 42 is BEEF
    And the register SP has the value 40

  Scenario: Test Pop opcode
    Given the bytes E1 at the position PC
    And the register SP set to the value 42
    And the bytes EF, BE at the position SP
    When the cpu has ticked 3 times
    Then the opcode was PopHL
    And the cpu has no action left
    And the register SP has the value 44
    And the register HL has the value BEEF
