Feature: Test Regs Endianness

  Scenario Outline: Test Regs u16 load
    Given the bytes <Bytes>, <ValueLow>, <ValueHigh> at the position PC
    When the cpu has ticked 4 times
    # currently the cucumber parser cannot handle touching template member see [cucumber-rs#161](https://github.com/cucumber-rs/cucumber/issues/161)
    Then the composite register <RegHigh> <RegLow> set to the value <ValueHigh> <ValueLow>
    And the u8 register <RegHigh> is set to <ValueHigh>
    And the u8 register <RegLow> is set to <ValueLow>

    Examples:
      | Bytes | RegHigh | RegLow | ValueHigh | ValueLow |
      | 01    | B       | C      | 66        | AE       |
      | 11    | D       | E      | 61        | 4B       |
      | 21    | H       | L      | 7C        | 7D       |
