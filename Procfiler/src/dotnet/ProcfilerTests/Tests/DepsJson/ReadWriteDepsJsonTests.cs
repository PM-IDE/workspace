using Autofac;
using Core.InstrumentalProfiler.DepsJson;
using ProcfilerTests.Core;
using TestsUtil;

namespace ProcfilerTests.Tests.DepsJson;

[TestFixture]
public class ReadWriteDepsJsonTests : TestWithContainerBase
{
  [Test]
  public void TestReadWrite()
  {
    var depsJsonsDirectory = Path.Combine(TestPaths.CreatePathToTestData(), "DepsJsons");
    var reader = Container.Resolve<IDepsJsonReader>();
    var writer = Container.Resolve<IDepsJsonWriter>();

    foreach (var file in Directory.GetFiles(depsJsonsDirectory))
    {
      var filePath = Path.Combine(depsJsonsDirectory, file);
      var depsJsonFile = reader.ReadOrThrowAsync(filePath).GetAwaiter().GetResult();
      using var ms = new MemoryStream();
      writer.WriteAsync(ms, depsJsonFile).GetAwaiter().GetResult();
      ms.Position = 0;
      using var sr = new StreamReader(ms);
      var json = sr.ReadToEnd();
      var originalJson = File.ReadAllText(filePath);
      Assert.That(json, Is.EqualTo(originalJson));
    }
  }
}