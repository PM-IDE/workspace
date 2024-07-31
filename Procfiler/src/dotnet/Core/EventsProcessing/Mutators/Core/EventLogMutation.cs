namespace Core.EventsProcessing.Mutators.Core;

public record EventLogMutation(string EventType);

public record AttributeRemovalMutation(string EventType, string AttributeName) : EventLogMutation(EventType);

public record AttributeRenameMutation(string EventType, string OldName, string NewName) : EventLogMutation(EventType);

public record AttributeToNameAppendMutation(
  string EventType,
  EventClassKind EventClassKind,
  string AttributeName,
  bool RemoveFromMetadata
) : EventLogMutation(EventType);

public record NewAttributeCreationMutation(string EventType, string AttributeName) : EventLogMutation(EventType);

public record AddEventMutation(string EventType) : EventLogMutation(EventType);

public record AddLifecycleTransitionAttributeMutation(string EventType, string LifecycleTransition) : EventLogMutation(EventType);

public record ActivityIdCreation(string EventType, string ActivityIdTemplate) : EventLogMutation(EventType);

public record EventTypeNameMutation(string EventType, string NewEventTypeName) : EventLogMutation(EventType);