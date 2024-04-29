using Bxes.Models.Domain;

namespace Bxes.Reader;

public readonly struct BxesReadContext
{
    public BinaryReader Reader { get; }
    public List<BxesValue> Values { get; }
    public List<KeyValuePair<uint, uint>> KeyValues { get; }


    public BxesReadContext(BinaryReader reader)
    {
        Reader = reader;
        Values = [];
        KeyValues = [];
    }

    public BxesReadContext(BinaryReader reader, List<BxesValue> values, List<KeyValuePair<uint, uint>> keyValues)
    {
        Reader = reader;
        Values = values;
        KeyValues = keyValues;
    }


    public BxesReadContext WithReader(BinaryReader reader) => new(reader, Values, KeyValues);
}