namespace Core.EventsProcessing.Mutators.Core.Passes;

public static class MultipleEventMutatorsPasses
{
  public const int MethodStartEndInserter = 0;
  public const int NotNeededMethodsRemove = 1000;
  public const int LastMultipleMutators = int.MaxValue;
}