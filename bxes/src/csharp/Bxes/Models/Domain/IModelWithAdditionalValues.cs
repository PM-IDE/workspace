namespace Bxes.Models.Domain;

public interface IModelWithAdditionalValues
{
  IEnumerable<BxesValue> EnumerateAdditionalValues();
}