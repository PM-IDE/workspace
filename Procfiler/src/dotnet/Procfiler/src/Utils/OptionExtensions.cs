using System.CommandLine.Binding;

namespace Procfiler.Utils;

public static class OptionExtensions
{
  public static T? GetDefaultValue<T>(this Option<T> option)
  {
    return (T?)((IValueDescriptor)option).GetDefaultValue();
  }
}