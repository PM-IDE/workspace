module IntegrationTests.UndefinedEvents

open NUnit.Framework
open Scripts.Core
open Scripts.Core.ProcfilerScriptsUtils
open Util

let createCustomConfig csprojPath outputPath : ICommandConfig =
    { SerializeUndefinedThreadEvents.Config.Base =
        { PathConfig =
            { OutputPath = outputPath
              CsprojPath = csprojPath }
          Repeat = 1
          Duration = 10_000
          WriteAllMetadata = true } }

let source () = knownProjectsNamesTestCaseSource

[<TestCaseSource("source")>]
let UndefinedEventsTest projectName =
    let doTest tempDir =
        let path = getCsprojPathFromSource projectName
        SerializeUndefinedThreadEvents.launchProcfilerCustomConfig path tempDir createCustomConfig

        doAssertionsForOneFile tempDir "UndefinedEvents"

    executeTestWithTempFolder doTest
