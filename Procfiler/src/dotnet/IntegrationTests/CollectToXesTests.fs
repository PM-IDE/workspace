module IntegrationTests.CollectToXesTests

open System.IO
open NUnit.Framework
open Scripts.Core
open Scripts.Core.ProcfilerScriptsUtils
open Util

let private createConfigInternal solutionPath outputPath : ICommandConfig =
    { CollectToXes.Config.Base =
        { PathConfig =
            { OutputPath = outputPath
              CsprojPath = solutionPath }
          Repeat = 1
          Duration = 10_000
          WriteAllMetadata = true } }

let source () = knownProjectsNamesTestCaseSource

[<TestCaseSource("source")>]
let CollectToXesTest projectName =
    let doTest tempDir =
        let csprojPath = getCsprojPathFromSource projectName
        let outputFileName = "data"
        let outputFilePath = Path.Combine(tempDir, $"{outputFileName}.xes")
        CollectToXes.launchProcfilerCustomConfig csprojPath outputFilePath createConfigInternal

        doAssertionsForOneFile tempDir outputFileName

    executeTestWithTempFolder doTest
