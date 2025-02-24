import os

gold_ext = '.gold'
tmp_ext = '.tmp'


class NoGoldFileException(Exception):
  def __init__(self, file_path: str):
    super().__init__(f'There is no gold file at {file_path}')


class TestValueNotEqualToGoldException(Exception):
  def __init__(self, expected_value: str, test_value: str):
    super().__init__(f'Expected: {expected_value}\n\nGot: {test_value}')


def execute_test_with_gold(gold_path: str, test_value: str):
  path_wo_extension = os.path.splitext(gold_path)[0]

  def write_tmp_file():
    directory = os.path.dirname(path_wo_extension)
    if not os.path.exists(directory):
      os.makedirs(directory, exist_ok=True)

    with open(path_wo_extension + tmp_ext, 'w') as fin:
      fin.write(test_value)

  if not os.path.exists(gold_path):
    write_tmp_file()
    raise NoGoldFileException(gold_path)

  with open(gold_path) as f:
    gold_value = ''.join(f.readlines())
    if gold_value != test_value:
      write_tmp_file()
      raise TestValueNotEqualToGoldException(gold_value, test_value)
