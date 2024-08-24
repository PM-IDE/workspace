using Core.Container;

namespace Procfiler.Core.SplitByMethod;

public interface IFullMethodNameBeautifier
{
  string Beautify(string fullMethodName);
}

[AppComponent]
public class FullMethodNameBeautifierImpl : IFullMethodNameBeautifier
{
  public string Beautify(string fullMethodName)
  {
    foreach (var invalidChar in Path.GetInvalidFileNameChars())
    {
      fullMethodName = fullMethodName.Replace(invalidChar.ToString(), string.Empty);
    }

    const int MaxMethodNameLength = 250;
    if (fullMethodName.Length > MaxMethodNameLength)
    {
      fullMethodName = fullMethodName[^MaxMethodNameLength..];
    }

    return fullMethodName;
  }
}