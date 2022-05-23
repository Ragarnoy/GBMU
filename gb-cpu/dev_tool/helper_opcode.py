# Tool that generate skel entry for opcode enum using a json database

from argparse import ArgumentParser
from json import load
import logging as log
import sys

log.basicConfig(level=log.DEBUG, stream=sys.stderr)

def parse_args():
  parser = ArgumentParser('helper opcode')
  parser.add_argument('--db', help='json opcode database', default='opcode_db.json')
  parser.add_argument('-b', '--begin', help='begining index', default=0, type=int)
  parser.add_argument('-e', '--end', help='ending index', default=0xff, type=int)
  parser.add_argument('--mode', choices=['unprefixed', 'cbprefixed'], help='opcode list to use', default='unprefixed')
  return parser.parse_args()

if __name__ == '__main__':
  args = parse_args()

  log.info(f"db={args.db}, begin={args.begin}, end={args.end}, mode={args.mode}")
  db = dict()

  with open(args.db) as f:
    db = load(f)

  opcodes = list()

  try:
    opcodes = db[args.mode.capitalize()]
  except Exception as e:
    raise ValueError(f'missing list type {args.mode} in database: {e}')

  import re

  log.debug(f'elements_number={len(opcodes)}')
  for i in range(args.begin, args.end + 1):
    element = opcodes[i]
    name = element['Name']
    branch_cycle = element['TCyclesBranch']
    no_branch_cycle = element['TCyclesNoBranch']

    if name == 'UNUSED':
      continue

    tag_name = re.sub(r'[iu](16|8)', r'\1', name)
    tag_name = re.sub(r'[()+,]', ' ', tag_name)
    tag_name = ''.join(c for c in tag_name.title() if not c.isspace())
    log.debug(f'name={name}, pascal={tag_name}, value={i:#02x}')

    timing = f'{branch_cycle//4}m' if branch_cycle == no_branch_cycle else f'{no_branch_cycle//4}-{branch_cycle//4}m'
    print(f'/// `{name}`: Foo . {timing}')
    print(f'{tag_name} = {i:#02x},')
