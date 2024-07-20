using Core.Container;
using Core.Utils;
using Mono.Cecil;
using Mono.Cecil.Cil;
using Mono.Collections.Generic;
using Procfiler.Core.Constants;
using Procfiler.Core.InstrumentalProfiler.DepsJson;
using Procfiler.Utils;

namespace Procfiler.Core.InstrumentalProfiler;

public enum InstrumentationKind
{
  None,
  MainAssembly,
  MainAssemblyAndReferences
}

public interface IDllMethodsPatcher
{
  Task PatchMethodStartEndAsync(string dllPath, InstrumentationKind instrumentationKind);
}

[AppComponent]
public class DllMethodsPatcher(IProcfilerLogger logger, IDepsJsonPatcher depsJsonPatcher) : IDllMethodsPatcher
{
  private const string ProcfilerEventSources = "ProcfilerEventSources";


  public async Task PatchMethodStartEndAsync(string dllPath, InstrumentationKind instrumentationKind)
  {
    if (instrumentationKind == InstrumentationKind.None) return;

    try
    {
      var directory = Path.GetDirectoryName(dllPath);
      Debug.Assert(directory is { });

      var cache = new SelfContainedTypeCache(logger, directory);
      cache.Initialize();

      var assembly = AssemblyDefinition.ReadAssembly(dllPath, new ReaderParameters
      {
        InMemory = true,
        ReadSymbols = true,
        ReadWrite = true
      });

      var assemblyWithPath = new AssemblyDefWithPath(assembly, dllPath);
      var patchedAssemblies = new List<AssemblyDefWithPath>();
      switch (instrumentationKind)
      {
        case InstrumentationKind.MainAssembly:
          PatchAssemblyMethods(assemblyWithPath, cache, patchedAssemblies);
          break;
        case InstrumentationKind.MainAssemblyAndReferences:
          PatchAssemblyMethodsWithReferences(assemblyWithPath, cache, patchedAssemblies);
          break;
        default:
          throw new ArgumentOutOfRangeException(nameof(instrumentationKind), instrumentationKind, null);
      }

      foreach (var (patchedAssembly, physicalPath) in patchedAssemblies)
      {
        patchedAssembly.Write(physicalPath);
      }

      await PatchDepsJsonAsync(assembly, dllPath);
    }
    catch (Exception ex) when (ex is not ArgumentOutOfRangeException)
    {
      logger.LogError(ex, "Failed to instrument code of {DllPath} or one of it's dependencies", dllPath);
    }
  }

  private async Task PatchDepsJsonAsync(AssemblyDefinition mainAssembly, string dllPath)
  {
    var directory = Path.GetDirectoryName(dllPath);
    Debug.Assert(directory is { });

    var assemblyName = mainAssembly.Name.Name;
    var depsJsonPath = Path.Combine(directory, $"{assemblyName}.deps.json");
    await depsJsonPatcher.AddAssemblyReferenceAsync(
      mainAssembly, depsJsonPath, ProcfilerEventSources, new Version(1, 0, 0));
  }

  private void PatchAssemblyMethodsWithReferences(
    AssemblyDefWithPath assemblyDefWithPath,
    SelfContainedTypeCache cache,
    List<AssemblyDefWithPath> patchedAssemblies)
  {
    var visited = new HashSet<string>();
    var queue = new Queue<AssemblyDefWithPath>();
    queue.Enqueue(assemblyDefWithPath);

    while (queue.Count > 0)
    {
      var currentAssembly = queue.Dequeue();
      if (visited.Contains(currentAssembly.Assembly.FullName)) continue;

      visited.Add(currentAssembly.Assembly.FullName);

      PatchAssemblyMethods(currentAssembly, cache, patchedAssemblies);

      foreach (var module in currentAssembly.Assembly.Modules)
      {
        foreach (var reference in module.AssemblyReferences)
        {
          if (cache.Assemblies.TryGetValue(reference.FullName, out var referenceAssembly))
          {
            queue.Enqueue(referenceAssembly);
          }
        }
      }
    }
  }

  private void PatchAssemblyMethods(
    AssemblyDefWithPath assemblyDefWithPath,
    SelfContainedTypeCache cache,
    List<AssemblyDefWithPath> patchedAssemblies)
  {
    var (assembly, path) = assemblyDefWithPath;
    if ((assembly.MainModule.Attributes & ModuleAttributes.ILOnly) == 0)
    {
      logger.LogWarning("Will not patch {Assembly} as it is not IL only", path);
      return;
    }

    try
    {
      logger.LogInformation("Patching assembly: {Name}", assembly.Name);
      var loggerType = cache.Types[InstrumentalProfilerConstants.MethodStartEndEventSourceType];

      var methodStartedLogger = loggerType.Methods.FirstOrDefault(IsLogMethodStartedMethod);
      var methodFinishedLogger = loggerType.Methods.FirstOrDefault(IsLogMethodFinishedMethod);

      Debug.Assert(methodStartedLogger is { });
      Debug.Assert(methodFinishedLogger is { });

      foreach (var moduleDefinition in assembly.Modules)
      {
        var methodStartLogReference = moduleDefinition.ImportReference(methodStartedLogger);
        var methodFinishedLogReference = moduleDefinition.ImportReference(methodFinishedLogger);

        foreach (var type in moduleDefinition.Types)
        {
          foreach (var method in type.Methods)
          {
            PatchMethodIfNeeded(method, methodStartLogReference, methodFinishedLogReference);
          }
        }
      }

      patchedAssemblies.Add(assemblyDefWithPath);
    }
    catch (Exception ex)
    {
      logger.LogError(ex, "Failed to instrument code of {Name}", assembly.Name);
    }
  }

  private void PatchMethodIfNeeded(
    MethodDefinition method, MethodReference methodStartedLogReference, MethodReference methodFinishedLogReference)
  {
    if (!method.IsManaged) return;
    if (!method.HasBody) return;

    PatchMethod(method, methodStartedLogReference, methodFinishedLogReference);
  }

  private void PatchMethod(
    MethodDefinition method, MethodReference methodStartedLogReference, MethodReference methodFinishedLogReference)
  {
    var processor = method.Body.GetILProcessor();
    var instructions = method.Body.Instructions;

    for (var i = instructions.Count - 1; i >= 0; i--)
    {
      if (ShouldInsertExitLogging(instructions, i))
      {
        InsertProcfilerLogBefore(i, processor, methodFinishedLogReference, false);
      }
    }

    InsertProcfilerLogBefore(0, processor, methodStartedLogReference, true);

    Instruction GetStartOfInsertedInstructions(Instruction context) => context.Previous.Previous.Previous.Previous;

    foreach (var instruction in instructions)
    {
      if (instruction.Operand is Instruction target && ShouldInsertExitLogging(target))
      {
        instruction.Operand = GetStartOfInsertedInstructions(target);
      }
    }

    foreach (var handler in method.Body.ExceptionHandlers)
    {
      if (ShouldInsertExitLogging(handler.HandlerEnd))
      {
        handler.HandlerEnd = GetStartOfInsertedInstructions(handler.HandlerEnd);
      }

      if (ShouldInsertExitLogging(handler.TryEnd))
      {
        handler.TryEnd = GetStartOfInsertedInstructions(handler.TryEnd);
      }
    }
  }

  private static bool IsLogMethodStartedMethod(MethodDefinition methodDefinition) =>
    methodDefinition.Name == InstrumentalProfilerConstants.LogMethodStartedMethodName &&
    IsMethodWithOneStringParameter(methodDefinition);

  private static bool IsLogMethodFinishedMethod(MethodDefinition methodDefinition) =>
    methodDefinition.Name == InstrumentalProfilerConstants.LogMethodFinishedMethodName &&
    IsMethodWithOneStringParameter(methodDefinition);

  private static bool IsMethodWithOneStringParameter(MethodDefinition methodDefinition) =>
    methodDefinition.Parameters.Count == 1 &&
    methodDefinition.Parameters[0].ParameterType.FullName == DotNetConstants.SystemString;

  private static bool IsConsoleWriteLineMethod(MethodDefinition methodDefinition) =>
    methodDefinition.Name == DotNetConstants.WriteLine &&
    IsMethodWithOneStringParameter(methodDefinition);

  private static bool ShouldInsertExitLogging(Instruction instruction) =>
    instruction.OpCode == OpCodes.Ret;

  private static bool ShouldInsertExitLogging(Collection<Instruction> instructions, int index) =>
    ShouldInsertExitLogging(instructions[index]);

  private static void InsertProcfilerLogBefore(
    int index, ILProcessor processor, MethodReference methodReference, bool entering)
  {
    var message = processor.Body.Method.FullName;

    var instruction = processor.Body.Instructions[index];
    if (index == 0 || processor.Body.Instructions[index - 1].OpCode != OpCodes.Nop)
    {
      processor.InsertBefore(instruction, Instruction.Create(OpCodes.Nop));
    }

    var loadStringInstruction = Instruction.Create(OpCodes.Ldstr, message);

    processor.InsertBefore(instruction, loadStringInstruction);
    var call = Instruction.Create(OpCodes.Call, methodReference);
    processor.InsertAfter(loadStringInstruction, call);

    processor.InsertAfter(call, Instruction.Create(OpCodes.Nop));
  }
}