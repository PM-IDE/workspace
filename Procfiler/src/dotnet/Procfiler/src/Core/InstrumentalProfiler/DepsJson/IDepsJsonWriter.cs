using Core.Container;
using Core.Utils.Json;

namespace Procfiler.Core.InstrumentalProfiler.DepsJson;

public interface IDepsJsonWriter
{
  Task WriteAsync(Stream stream, DepsJsonFile file);
}

public static class ExtensionsForIDepsJsonWriter
{
  public static async Task WriteAsync(this IDepsJsonWriter writer, string path, DepsJsonFile file)
  {
    await using var fs = File.OpenWrite(path);
    await writer.WriteAsync(fs, file);
  }
}

[AppComponent]
public class DepsJsonWriterImpl : IDepsJsonWriter
{
  public async Task WriteAsync(Stream stream, DepsJsonFile file)
  {
    await using var writer = new Utf8JsonWriter(stream, new JsonWriterOptions
    {
      Indented = true
    });

    using var _ = new StartEndObjectCookie(writer);

    WriteRuntimeTarget(writer, file.RuntimeTarget);
    WriteCompilationOptions(writer, file.CompilationOptions);
    WriteTargets(writer, file.Targets);
    WriteLibraries(writer, file.Libraries);
    WriteRuntimes(writer, file.Runtimes);
  }

  private static void WriteRuntimes(Utf8JsonWriter writer, Runtimes runtimes)
  {
    using var _ = new StartEndObjectCookie(writer, DepsJsonConstants.Runtimes);
    foreach (var runtime in runtimes.RuntimesList)
    {
      using var __ = new StartEndArrayCookie(writer, runtime.Rid);
      foreach (var fallback in runtime.Fallbacks)
      {
        writer.WriteStringValue(fallback);
      }
    }
  }

  private static void WriteLibraries(Utf8JsonWriter writer, Libraries libraries)
  {
    using var _ = new StartEndObjectCookie(writer, DepsJsonConstants.Libraries);
    foreach (var libraryEntry in libraries.LibrariesList)
    {
      WriteLibraryEntry(writer, libraryEntry);
    }
  }

  private static void WriteLibraryEntry(Utf8JsonWriter writer, LibraryEntry entry)
  {
    using var _ = new StartEndObjectCookie(writer, entry.Name);

    if (entry.Type is { }) writer.WriteString(DepsJsonConstants.Type, entry.Type);
    if (entry.Serviceable.HasValue) writer.WriteBoolean(DepsJsonConstants.Serviceable, entry.Serviceable.Value);
    if (entry.HashPath is { }) writer.WriteString(DepsJsonConstants.HashPath, entry.HashPath);
    if (entry.Path is { }) writer.WriteString(DepsJsonConstants.Path, entry.Path);

    writer.WriteString(DepsJsonConstants.Sha512, entry.Sha512 ?? string.Empty);
  }

  private static void WriteCompilationOptions(Utf8JsonWriter writer, CompilationOptions options)
  {
    using var _ = new StartEndObjectCookie(writer, DepsJsonConstants.CompilationOptions);
    foreach (var option in options.Options)
    {
      using var __ = new StartEndObjectCookie(writer, option);
    }
  }

  private static void WriteRuntimeTarget(Utf8JsonWriter writer, TopLevelRuntimeTarget runtimeTarget)
  {
    using var _ = new StartEndObjectCookie(writer, DepsJsonConstants.RuntimeTarget);
    writer.WriteString(DepsJsonConstants.Name, runtimeTarget.NameWithVersion.ToString());
    writer.WriteString(DepsJsonConstants.Signature, runtimeTarget.Signature);
  }

  private static void WriteTargets(Utf8JsonWriter writer, Targets targets)
  {
    using var _ = new StartEndObjectCookie(writer, DepsJsonConstants.Targets);
    foreach (var target in targets.TargetsList)
    {
      using var __ = new StartEndObjectCookie(writer, target.NameWithVersion.ToString());
      foreach (var dependency in target.Dependencies)
      {
        using var ___ = new StartEndObjectCookie(writer, dependency.Name);
        if (dependency.Dependencies.Count > 0)
        {
          using (new StartEndObjectCookie(writer, DepsJsonConstants.Dependencies))
          {
            foreach (var dependencyOfTargetDependency in dependency.Dependencies)
            {
              var name = dependencyOfTargetDependency.NameWithVersion.Name;
              var version = dependencyOfTargetDependency.NameWithVersion.Version;
              writer.WriteString(name, version);
            }
          }
        }

        if (dependency.Native.Count > 0)
        {
          using (new StartEndObjectCookie(writer, DepsJsonConstants.Native))
          {
            foreach (var fileInfo in dependency.Native)
            {
              WriteFileInfo(writer, fileInfo);
            }
          }
        }

        if (dependency.Runtime.Count > 0)
        {
          using (new StartEndObjectCookie(writer, DepsJsonConstants.Runtime))
          {
            foreach (var fileInfo in dependency.Runtime)
            {
              WriteFileInfo(writer, fileInfo);
            }
          }
        }

        if (dependency.RuntimeTargets.Count > 0)
        {
          using (new StartEndObjectCookie(writer, DepsJsonConstants.RuntimeTargets))
          {
            foreach (var runtimeTarget in dependency.RuntimeTargets)
            {
              WriteRuntimeTarget(writer, runtimeTarget);
            }
          }
        }
      }
    }
  }

  private static void WriteRuntimeTarget(Utf8JsonWriter writer, RuntimeTarget runtimeTarget)
  {
    using var _ = new StartEndObjectCookie(writer, runtimeTarget.Name);
    writer.WriteString(DepsJsonConstants.Name, runtimeTarget.Rid);
    writer.WriteString(DepsJsonConstants.AssetType, runtimeTarget.AssetType);
    writer.WriteString(DepsJsonConstants.AssemblyVersion, runtimeTarget.AssemblyVersion.ToString());
  }

  private static void WriteFileInfo(Utf8JsonWriter writer, FileInfo fileInfo)
  {
    using var _ = new StartEndObjectCookie(writer, fileInfo.Name);
    if (fileInfo.AssemblyVersion is { } assemblyVersion)
    {
      writer.WriteString(DepsJsonConstants.AssemblyVersion, assemblyVersion.ToString());
    }

    if (fileInfo.FileVersion is { } fileVersion)
    {
      writer.WriteString(DepsJsonConstants.FileVersion, fileVersion.ToString());
    }
  }
}