# Tool that help putting bulk comment above opcode enum

import logging as log
import sys

log.basicConfig(level=log.DEBUG, stream=sys.stderr)

def parse_arg():
  from argparse import ArgumentParser

  parser = ArgumentParser('helper')
  parser.add_argument("file", help="file to operate substitution")
  parser.add_argument("opcode", help="opcode name")
  parser.add_argument("description", help="opcode description")
  parser.add_argument("-r", "--num-regs", help="number of regs to catch", type=int, default=1)
  return parser.parse_args()

def process_file(file: str, opcode: str, description: str, reg_count: int):
  import fileinput
  import re

  opcode_tag = opcode.capitalize()
  opcode_desc = opcode.upper()
  line_re = rf'    (?P<tag>{opcode_tag})(?P<regs>(?:HL|[ABCDEHL]){{{reg_count},{reg_count}}}) = 0x(?P<val>[0-9a-f]){{2,2}},'
  reg_re = r'(HL|[ABCDEHL])' * (reg_count)

  log.debug(f'file={file}, opcode={opcode}, description={description}, reg_count={reg_count}')

  with fileinput.input(files=file, inplace=True) as f:
    for line in f:
      m = re.match(line_re, line)
      if m is not None:
        log.debug(f'line="{line[:-1]}"')
        regs_str = m.group('regs')
        regs_m = re.match(reg_re, regs_str)
        regs_list = list(map(lambda reg: '(HL)' if reg == 'HL' else reg, regs_m.groups()))
        log.debug(f'tag={m.group("tag")}, regs={regs_list}, val={m.group("val")}')
        regs = ', '.join(regs_list)
        formated_desc = description.format(*regs_list)
        desc = f'   /// `{opcode_desc} {regs}`: {formated_desc}'
        log.debug(f'desc="{desc}"')
        print(desc)
      print(line, end='')


if __name__ == '__main__':
  arg = parse_arg()

  process_file(arg.file, arg.opcode.capitalize(), arg.description, arg.num_regs)
