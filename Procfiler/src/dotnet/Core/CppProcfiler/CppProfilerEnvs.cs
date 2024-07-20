namespace Core.CppProcfiler;

public static class CppProfilerEnvs
{
  public const string ShadowStackDebugSavePath = "PROCFILER_DEBUG_SAVE_CALL_STACKS_PATH";
  public const string EnableConsoleLogging = "PROCFILER_ENABLE_CONSOLE_LOGGING";
  public const string BinaryStacksSavePath = "PROCFILER_BINARY_SAVE_STACKS_PATH";
  public const string EventPipeSaveStacks = "PROCFILER_EVENT_PIPE_SAVE_STACKS";
  public const string MethodsFilterRegex = "PROCFILER_FILTER_METHODS_REGEX";
  public const string MethodsFilteringDuringRuntime = "PROCFILER_FILTER_METHODS_DURING_RUNTIME";
  public const string UseSeparateBinStacksFiles = "PROCFILER_USE_SEPARATE_BINSTACKS_FILES";
  public const string OnlineSerialization = "PROCFILER_ONLINE_SERIALIZATION";
}