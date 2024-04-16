namespace Procfiler.Core.Constants.TraceEvents;

public static partial class TraceEventsConstants
{
  public const string MethodInliningSucceeded = "Method/InliningSucceeded";

  public const string MethodInliningSucceededInlineeName = "InlineeName";
  public const string MethodInliningSucceededInlineeNamespace = "InlineeNamespace";

  public const string MethodInliningFailed = "Method/InliningFailed";

  public const string MethodJittingStarted = "Method/JittingStarted";

  public const string MethodId = "MethodID";
  public const string MethodSignature = "MethodSignature";
  public const string MethodName = "MethodName";
  public const string MethodNamespace = "MethodNamespace";

  public const string MethodLoadVerbose = "Method/LoadVerbose";
  public const string MethodDetails = "Method/MethodDetails";

  public const string MethodMemoryAllocatedForJitCode = "Method/MemoryAllocatedForJitCode";
  public const string MethodUnloadVerbose = "Method/UnloadVerbose";
  public const string MethodTailCallSucceeded = "Method/TailCallSucceeded";
  public const string MethodTailCallFailed = "Method/TailCallFailed";
  public const string MethodR2RGetEntryPointStart = "Method/R2RGetEntryPointStart";
  public const string MethodR2RGetEntryPoint = "Method/R2RGetEntryPoint";

  public const string MethodBeingCompiledNamespace = "MethodBeingCompiledNamespace";
  public const string MethodBeingCompiledName = "MethodBeingCompiledName";
}