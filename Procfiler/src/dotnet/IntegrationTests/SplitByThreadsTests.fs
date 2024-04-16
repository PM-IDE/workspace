module IntegrationTests.SplitByThreadsTests

open System
open System.IO
open NUnit.Framework
open Scripts.Core
open Scripts.Core.ProcfilerScriptsUtils
open Util

let private createConfigInternal solutionPath outputPath : ICommandConfig =
    { SplitByThreads.Config.PathConfig =
        { OutputPath = outputPath
          CsprojPath = solutionPath } }

let source () = knownProjectsNamesTestCaseSource

[<TestCaseSource("source")>]
let SplitByThreadsTest projectName =
    let doTest tempDir =
        let csprojPath = getCsprojPathFromSource projectName
        SplitByThreads.launchProcfilerCustomConfig csprojPath tempDir createConfigInternal

        let files = Directory.GetFiles(tempDir)
        let mutable threadIds = []

        files
        |> Array.iter (fun filePath ->
            Assert.That(FileInfo(filePath).Length, Is.GreaterThan 0)

            let mutable threadId = 0

            if Int32.TryParse(Path.GetFileNameWithoutExtension(filePath), &threadId) then
                threadIds <- threadIds @ [ threadId ])

        let fileNames = files |> Array.map Path.GetFileNameWithoutExtension |> Set.ofArray

        threadIds
        |> List.iter (fun threadId -> Assert.That(fileNames.Contains $"stacks_{threadId}"))

    executeTestWithTempFolder doTest
