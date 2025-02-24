using System.Text.Json;
using Autofac;
using Core.Builder;
using ProcfilerEventSources;
using ProcfilerTests.Core;
using TestsUtil;

namespace ProcfilerTests.Tests.ProcfilerEventPipeLoggerTests;

[TestFixture]
public class ProcfilerEventPipeLoggerTest : GoldProcessBasedTest
{
  [Test]
  public void DoTest()
  {
    var context = KnownSolution.ProcfilerEventPipeLogger.CreateDefaultContext();
    var builder = Container.Resolve<IDotnetProjectBuilder>();
    var result = builder.TryBuildDotnetProject(context.ProjectBuildInfo with
    {
      CsprojPath = TestPaths.CreatePathToProcfilerLoggerProviderProject(),
      ClearArtifacts = false,
      TempPath = ProjectBuildOutputPath.DefaultNetFolder,
      AdditionalBuildArgs = null
    });

    Assert.That(result, Is.Not.Null);

    ExecuteTestWithGold(
      context,
      events => string.Join(
        "\n",
        events.Events
          .Where(e => e.Event.EventName is nameof(ProcfilerBusinessEventsSource.BusinessEvent))
          .Select(e => e.Event)
          .OrderBy(e => e.Time.QpcStamp)
          .Select(e => $"{e.EventName} {JsonSerializer.Serialize(e.Metadata)}")
      )
    );
  }
}