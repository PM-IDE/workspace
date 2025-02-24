from dataclasses import dataclass


class GraphNode:
  def __init__(self, name: str):
    self.name: str = name
    self.child_nodes: list['GraphNode'] = []

  def _serialize(self):
    pass

  def __str__(self):
    if len(self.child_nodes) == 0:
      return self._serialize()

    result = f'{self._serialize()}'
    for child in self.child_nodes:
      result += f'({str(child)})'

    return result


@dataclass
class SubArrayInEventLog:
  first_pos: int
  length: int
