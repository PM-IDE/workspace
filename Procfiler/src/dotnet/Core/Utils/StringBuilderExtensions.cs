using System.Collections;
using System.Text;

namespace Core.Utils;

public static class StringBuilderExtensions
{
  extension(StringBuilder sb)
  {
    public StringBuilder LogPrimitiveValue<T>(string name, T value) where T : struct =>
      sb.Append(name).Append(" = ").Append(value)
        .AppendNewLine();

    public StringBuilder LogDictionary<TKey, TValue>(string name, Dictionary<TKey, TValue> map)
      where TKey : notnull
    {
      sb.Append(name).Append(':')
        .AppendNewLine()
        .Append('{')
        .AppendNewLine();

      foreach (var (key, value) in map)
      {
        sb.Append('\t').Append(key).Append(" = ").Append(SerializeValue(value))
          .AppendNewLine();
      }

      return sb.Append('}')
        .AppendNewLine();
    }

    public StringBuilder AppendSpace() => sb.Append(' ');
    public PairedCharCookie AppendBraces() => new(sb, '(', ')');
  }

  public static string SerializeValue<T>(T value)
  {
    if (value is null) return string.Empty;
    if (value is string @string) return @string;
    if (value is IEnumerable<char> chars) return new string(chars.ToArray());
    if (value is not IEnumerable enumerable) return value.ToString() ?? string.Empty;

    var sb = new StringBuilder();
    var any = false;
    const string Delimiter = "  ";

    sb.Append('[');
    foreach (var item in enumerable)
    {
      any = true;
      sb.Append(item).Append(Delimiter);
    }

    if (any)
    {
      sb.Remove(sb.Length - Delimiter.Length, Delimiter.Length);
    }

    return sb.Append(']').ToString();
  }

  extension(StringBuilder sb)
  {
    public StringBuilder AppendTab() => sb.Append('\t');
    public StringBuilder AppendNewLine() => sb.Append('\n');
  }
}

public readonly struct PairedCharCookie : IDisposable
{
  private readonly StringBuilder myStringBuilder;
  private readonly char myCloseChar;


  public PairedCharCookie(StringBuilder stringBuilder, char openChar, char closeChar)
  {
    stringBuilder.Append(openChar);
    myStringBuilder = stringBuilder;
    myCloseChar = closeChar;
  }


  public void Dispose()
  {
    myStringBuilder.Append(myCloseChar);
  }
}