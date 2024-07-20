using System.Text.Json;
using Core.Container;
using Core.Utils;

namespace Core.InstrumentalProfiler.DepsJson;

public interface IDepsJsonReader
{
  Task<DepsJsonFile> ReadOrThrowAsync(string path);
}

[AppComponent]
public class DepsJsonReaderImpl : IDepsJsonReader
{
  public async Task<DepsJsonFile> ReadOrThrowAsync(string path)
  {
    PathUtils.ThrowIfNotExists(path);
    await using var fs = File.OpenRead(path);
    var document = await JsonDocument.ParseAsync(fs);

    var file = new DepsJsonFile();
    foreach (var property in document.RootElement.EnumerateObject())
    {
      Action<DepsJsonFile, JsonProperty>? action = property.Name switch
      {
        DepsJsonConstants.RuntimeTarget => SetRuntimeTarget,
        DepsJsonConstants.Targets => SetTargets,
        DepsJsonConstants.Libraries => SetLibraries,
        DepsJsonConstants.Runtimes => SetRuntimes,
        DepsJsonConstants.CompilationOptions => SetCompilationOptions,
        _ => null
      };

      action?.Invoke(file, property);
    }

    return file;
  }

  private static void SetCompilationOptions(DepsJsonFile file, JsonProperty property)
  {
    file.CompilationOptions = new CompilationOptions
    {
      Options = property.Value.EnumerateObject().Select(obj => obj.Value.GetStringValueOrThrow()).ToList()
    };
  }

  private static void SetRuntimes(DepsJsonFile file, JsonProperty property)
  {
    file.Runtimes = new Runtimes
    {
      RuntimesList = property.Value.EnumerateObject().Select(ParseRuntime).ToList()
    };
  }

  private static void SetLibraries(DepsJsonFile file, JsonProperty property)
  {
    file.Libraries = new Libraries
    {
      LibrariesList = property.Value.EnumerateObject().Select(ParseLibrary).ToList()
    };
  }

  private static void SetTargets(DepsJsonFile file, JsonProperty property)
  {
    file.Targets = new Targets
    {
      TargetsList = property.Value.EnumerateObject().Select(ParseTargetItem).ToList()
    };
  }

  private static void SetRuntimeTarget(DepsJsonFile file, JsonProperty property)
  {
    file.RuntimeTarget = new TopLevelRuntimeTarget
    {
      Signature = property.Value.GetPropertyStringValueOrThrow(DepsJsonConstants.Signature),
      NameWithVersion = NameWithVersion.Parse(property.Value.GetPropertyStringValueOrThrow(DepsJsonConstants.Name))
    };
  }

  private static Runtime ParseRuntime(JsonProperty property) => new()
  {
    Rid = property.Name,
    Fallbacks = property.Value.EnumerateArray().Select(el => el.GetStringValueOrThrow()).ToList()
  };

  private static LibraryEntry ParseLibrary(JsonProperty property) => new()
  {
    Name = property.Name,
    Path = property.Value.TryGetPropertyStringValue(DepsJsonConstants.Path),
    Serviceable = property.Value.TryGetPropertyBoolValue(DepsJsonConstants.Serviceable),
    Sha512 = property.Value.TryGetPropertyStringValue(DepsJsonConstants.Sha512),
    Type = property.Value.TryGetPropertyStringValue(DepsJsonConstants.Type),
    HashPath = property.Value.TryGetPropertyStringValue(DepsJsonConstants.HashPath)
  };

  private static TargetItem ParseTargetItem(JsonProperty property) =>
    new()
    {
      NameWithVersion = NameWithVersion.Parse(property.Name),
      Dependencies = property.Value.EnumerateObject().Select(ParseTargetDependency).ToList()
    };

  private static TargetDependency ParseTargetDependency(JsonProperty dependency)
  {
    JsonElement? dependencies = null;
    JsonElement? runtime = null;
    JsonElement? runtimeTargets = null;
    JsonElement? native = null;

    foreach (var property in dependency.Value.EnumerateObject())
    {
      switch (property.Name)
      {
        case DepsJsonConstants.Dependencies:
          dependencies = property.Value;

          break;
        case DepsJsonConstants.Runtime:
          runtime = property.Value;

          break;
        case DepsJsonConstants.Native:
          native = property.Value;

          break;
        case DepsJsonConstants.RuntimeTargets:
          runtimeTargets = property.Value;

          break;
      }
    }

    return new TargetDependency
    {
      Name = dependency.Name,
      Dependencies = SelectFrom(dependencies, ParseDependencyOfTargetDependency),
      Native = SelectFrom(native, ParseFileInfo).ToList(),
      Runtime = SelectFrom(runtime, ParseFileInfo).ToList(),
      RuntimeTargets = SelectFrom(runtimeTargets, ParseRuntimeTargets).ToList()
    };
  }

  private static List<T> SelectFrom<T>(JsonElement? element, Func<JsonProperty, T> selector) =>
    element?.EnumerateObject().Select(selector).ToList() ??
    EmptyCollections<T>.EmptyList;

  private static DependencyOfTargetDependency ParseDependencyOfTargetDependency(JsonProperty property) => new()
  {
    NameWithVersion = NameWithVersion.Create(property.Name, property.Value.GetStringValueOrThrow())
  };

  private static RuntimeTarget ParseRuntimeTargets(JsonProperty property) => new()
  {
    Name = property.Name,
    Rid = property.Value.GetPropertyStringValueOrThrow(DepsJsonConstants.Rid),
    AssetType = property.Value.GetPropertyStringValueOrThrow(DepsJsonConstants.AssetType),
    AssemblyVersion = ParseVersionOrThrow(property.Value.GetProperty(DepsJsonConstants.AssemblyVersion))
  };

  private static FileInfo ParseFileInfo(JsonProperty property) => new()
  {
    Name = property.Name,
    AssemblyVersion = TryParseVersion(property.Value.GetPropertyOrNull(DepsJsonConstants.AssemblyVersion)),
    FileVersion = TryParseVersion(property.Value.GetPropertyOrNull(DepsJsonConstants.FileVersion))
  };

  private static Version ParseVersionOrThrow(JsonElement? element)
  {
    if (TryParseVersion(element) is not { } version)
    {
      throw new FormatException(element?.GetString());
    }

    return version;
  }

  private static Version? TryParseVersion(JsonElement? element)
  {
    if (element is null) return null;

    return Version.TryParse(element.Value.GetString(), out var version) ? version : null;
  }
}