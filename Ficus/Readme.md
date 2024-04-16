## Ficus

Ficus is a python package where some Process Mining algorithms are implemented.
Now this project is an active development stage.

Algorithms executions in Ficus use pipelines (similar to sklearn):

```python

    Pipeline(
        ReadLogFromXes(),
        RemoveEventsFromLogPredicate(procfiler_filter_predicate),
        RemoveEventsFromLogPredicate(lambda x: 'BYTE[]' in x[concept_name]),
        IfPipeline(should_filter, Pipeline(
            PosEntropyDirectFilter(threshold, max_events_to_remove=5),
        )),
        FilterTracesByCount(min_event_in_trace_count=3),
        DiscoverActivitiesFromTandemArrays(array_kind=TandemArrayKind.PrimitiveArray,
                                           activity_in_trace_filter=activity_in_trace_filter,
                                           activity_level=0),
        DrawFullActivitiesDiagram(plot_legend=True),
        CreateLogFromActivities(use_hashes_as_names=use_hashes_as_names),
        ClearActivities(),
        DiscoverActivitiesForSeveralLevels([default_class_extractor, strongest_class_extractor_evt],
                                           discovering_strategy=ActivitiesDiscoveryStrategy.DiscoverFromAllTraces),
        CalculatePercentageOfUnattachedEvents(),
        DrawFullActivitiesDiagram(plot_legend=False),
        CreateLogFromActivities(undef_evt_strategy, use_hashes_as_names=use_hashes_as_names),
        ClearActivities(),
        DiscoverActivitiesForSeveralLevels([default_class_extractor],
                                           discovering_strategy=ActivitiesDiscoveryStrategy.DiscoverFromAllTraces),
        DrawFullActivitiesDiagram(plot_legend=True),
        CreateLogsForActivities(class_extractor=default_class_extractor, activity_level=0),
        ExecuteWithEachActivityLog(Pipeline(
            SubstituteUnderlyingEvents(),
            RemoveEventsFromLogPredicate(methods_filter_predicate),
            FilterTracesByVariants(),
            TracesDiversityDiagram(plot_legend=True),
            RemoveLifecycleTransitionsAttributes(),
            SaveEventLog(activity_log_save_path_creator)
        )),
    )(path)

```