from typing import Callable

import graphviz

from ....analysis.common.common_models import GraphNode

GraphAttributesSetter = Callable[[graphviz.Digraph], None]
NodeNameCreator = Callable[[GraphNode], str]
