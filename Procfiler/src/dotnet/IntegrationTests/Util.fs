module IntegrationTests.Util

open System.IO
open NUnit.Framework
open TestsUtil

let getCsprojPathFromSource solutionName =
    Path.Combine(TestPaths.CreatePathToSolutionsSource(), solutionName, $"{solutionName}.csproj")

let knownProjectsNamesTestCaseSource =
    KnownSolution.AllSolutions |> Seq.map (fun solution -> solution.Name)

let executeTestWithTempFolder test =
    let tempDir = Directory.CreateTempSubdirectory().FullName

    try
        test tempDir
    finally
        Directory.Delete(tempDir, true)


let doAssertionsForOneFile tempDir expectedFileName =
    let files = Directory.GetFiles(tempDir)

    Assert.That(files.Length, Is.EqualTo 1)
    Assert.That(Path.GetExtension files[0], Is.EqualTo ".xes")
    Assert.That(Path.GetFileNameWithoutExtension files[0], Is.EqualTo expectedFileName)
    Assert.That(FileInfo(files[0]).Length, Is.GreaterThan(0))
