from typing import Optional

import graphviz

from .petri_net import draw_formalism


class Graph:
    def __init__(self):
        self.nodes = []
        self.edges = []

class GraphNode:
    def __init__(self, id: int, data: Optional[str]):
        self.id = id
        self.data = data

class GraphEdge:
    def __init__(self, from_node: int, to_node: int, data: Optional[str]):
        self.from_node = from_node
        self.to_node = to_node
        self.data = data

def draw_graph(graph: Graph,
               name: str = 'petri_net',
               background_color: str = 'white',
               engine='dot',
               export_path: Optional[str] = None,
               rankdir: str = 'LR'):
    def draw_func(g: graphviz.Digraph):
        for node in graph.nodes:
            g.node(str(node.id), xlabel=node.data, style='filled', border='1', shape='circle')

        for edge in graph.edges:
            g.edge(str(edge.from_node), str(edge.to_node), edge.data)

    draw_formalism(draw_func,
                   name=name,
                   background_color=background_color,
                   engine=engine,
                   export_path=export_path,
                   rankdir=rankdir)
