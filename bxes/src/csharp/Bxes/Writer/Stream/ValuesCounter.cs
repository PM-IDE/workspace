using Bxes.Models.Domain;

namespace Bxes.Writer.Stream;

internal readonly struct ValuesCounter()
{
    private readonly Dictionary<BxesValue, int> myValuesCounts = new();
    private readonly Dictionary<AttributeKeyValue, int> myKeyValuesCounts = new();


    public IEnumerable<BxesValue> CreateValuesIndices() => CreateIndices(myValuesCounts);
    public IEnumerable<AttributeKeyValue> CreateKeyValueIndices() => CreateIndices(myKeyValuesCounts);
  
    private static IEnumerable<T> CreateIndices<T>(Dictionary<T, int> map) where T : notnull => 
        OrderByCount(map);

    private static IEnumerable<T> OrderByCount<T>(Dictionary<T, int> map) where T : notnull => 
        map.OrderByDescending(pair => pair.Value).Select(pair => pair.Key);

    public void HandleValue(BxesValue value) => InitOrIncrease(myValuesCounts, value);

    public void HandleKeyValue(AttributeKeyValue kv)
    {
        InitOrIncrease(myKeyValuesCounts, kv);
        HandleValue(kv.Key);
        HandleValue(kv.Value);
    }

    private static void InitOrIncrease<T>(IDictionary<T, int> map, T value) where T : notnull
    {
        if (map.TryGetValue(value, out var count))
        {
            map[value] = count + 1;
        }
        else
        {
            map[value] = 1;
        }
    }

    public XesToBxesConversionStatistics ToStatistics() => new()
    {
        Values = myValuesCounts,
        Attributes = myKeyValuesCounts
    };
}