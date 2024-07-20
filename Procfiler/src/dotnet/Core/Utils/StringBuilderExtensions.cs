using System.Collections;
using System.Text;

namespace Core.Utils;

public static class StringBuilderExtensions
{
  public static StringBuilder LogPrimitiveValue<T>(this StringBuilder sb, string name, T value) where T : struct =>
    sb.Append(name).Append(" = ").Append(value)
      .AppendNewLine();

  public static StringBuilder LogDictionary<TKey, TValue>(this StringBuilder sb, string name, Dictionary<TKey, TValue> map)
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

  public static StringBuilder AppendSpace(this StringBuilder sb) => sb.Append(' ');

  public static PairedCharCookie AppendBraces(this StringBuilder sb) => new(sb, '(', ')');

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

  public static StringBuilder AppendTab(this StringBuilder sb) => sb.Append('\t');

  public static StringBuilder AppendNewLine(this StringBuilder sb) => sb.Append('\n');
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