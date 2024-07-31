using Core.GlobalData;

namespace Procfiler.Core.Collector;

public interface IGlobalDataWithStacks : IGlobalData
{
  IShadowStacks Stacks { get; }
}