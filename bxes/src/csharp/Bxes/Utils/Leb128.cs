namespace Bxes.Utils;

//https://github.com/rzubek/mini-leb128/blob/master/LEB128.cs
public static class Leb128
{
  private const long SignExtendMask = -1L;
  private const int Int64BitSize = sizeof(long) * 8;

  public static void WriteLeb128Signed(this BinaryWriter writer, long value)
  {
    var more = true;

    while (more)
    {
      var chunk = (byte)(value & 0x7fL);
      value >>= 7;

      var signBitSet = (chunk & 0x40) != 0;
      more = !((value == 0 && !signBitSet) || (value == -1 && signBitSet));
      if (more)
      {
        chunk |= 0x80;
      }

      writer.Write(chunk);
    }
  }

  public static void WriteLeb128Unsigned(this BinaryWriter writer, ulong value)
  {
    var more = true;

    while (more)
    {
      var chunk = (byte)(value & 0x7fUL);
      value >>= 7;

      more = value != 0;
      if (more)
      {
        chunk |= 0x80;
      }

      writer.Write(chunk);
    }
  }

  public static long ReadLeb128Signed(this BinaryReader reader)
  {
    long value = 0;
    var shift = 0;
    bool more = true, signBitSet = false;

    while (more)
    {
      var next = reader.ReadByte();

      more = (next & 0x80) != 0;
      signBitSet = (next & 0x40) != 0;

      var chunk = next & 0x7fL;
      value |= chunk << shift;
      shift += 7;
    }

    if (shift < Int64BitSize && signBitSet)
    {
      value |= SignExtendMask << shift;
    }

    return value;
  }

  public static ulong ReadLeb128Unsigned(this BinaryReader reader)
  {
    ulong value = 0;
    var shift = 0;
    var more = true;

    while (more)
    {
      var next = reader.ReadByte();

      more = (next & 0x80) != 0;
      var chunk = next & 0x7fUL;
      value |= chunk << shift;
      shift += 7;
    }

    return value;
  }
}