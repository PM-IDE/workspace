using Procfiler.Core.Collector;

namespace Procfiler.Core.CppProcfiler.ShadowStacks;

public interface ICppShadowStacks : IShadowStacks
{
  IEnumerable<ICppShadowStack> EnumerateStacks();
  ICppShadowStack? FindShadowStack(long managedThreadId);
}