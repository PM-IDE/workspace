## Ficus

Ficus is a modern process mining toolkit that consists two parts: backend written in Rust and a Python client.
For Ficus to work you will need to install additional tools, such as `graphviz`.

Now the main scenario is to create a `Pipeline` that declaratively describes process mining processing pipeline.
The input of the pipeline is initial context, where usually names event log or a path to `xes` or `bxes` event log is specified.

For the pipeline to run, Ficus backend should be launched:
- Either use `__call__` or `execute_docker` methods to pull the Ficus docker image, launch a container with Ficus backend and remove it
  after pipeline finished its execution

  ```python
  Pipeline(
      UseNamesEventLog(),
      AddStartEndArtificialEvents(),
      DiscoverPetriNetHeuristic(dependency_threshold=0.5, loop_length_two_threshold=0.5),
      EnsureInitialMarking(),
      AnnotatePetriNetWithTraceFrequency(show_places_names=False, rankdir='TB'),
  )({
      'names_event_log': NamesLogContextValue([
          ["A", "B", "D"],
          ["A", "B", "C", "B", "D"],
          ["A", "B", "C", "B", "C", "B", "D"],
      ])
  })
  
  ```

- Or use `execute` method that accepts ficus backend url (that is launched manually) and the initial context.

  ```python
  Pipeline(
      UseNamesEventLog(),
      AddStartEndArtificialEvents(),
      DiscoverPetriNetHeuristic(dependency_threshold=0.5, loop_length_two_threshold=0.5),
      EnsureInitialMarking(),
      AnnotatePetriNetWithTraceFrequency(show_places_names=False, rankdir='TB'),
  ).execute('localhost:8080', {
      'names_event_log': NamesLogContextValue([
          ["A", "B", "D"],
          ["A", "B", "C", "B", "D"],
          ["A", "B", "C", "B", "C", "B", "D"],
      ])
  })
  ```