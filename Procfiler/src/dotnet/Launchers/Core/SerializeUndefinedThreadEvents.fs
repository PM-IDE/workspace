namespace Scripts.Core

open Scripts.Core.ProcfilerScriptsUtils

module SerializeUndefinedThreadEvents =
    type Config =
        { Base: ConfigBase }

        interface ICommandConfig with
            member this.CreateArguments() =
                [ "undefined-events-to-xes" ] |> this.Base.AddArguments


    let private createConfig csprojPath outputPath : ICommandConfig =
        { Base = createDefaultConfigBase csprojPath outputPath }

    let launchProcfilerCustomConfig csprojPath outputPath createCustomConfig =
        ensureEmptyDirectory outputPath |> ignore
        launchProcfiler csprojPath outputPath createCustomConfig

    let launchProcfiler csprojPath outputPath =
        launchProcfilerCustomConfig csprojPath outputPath createConfig

    let launchProcfilerOnFolderOfSolutions pathToFolderWithSolutions outputPath =
        let pathsToCsprojFiles = getAllCsprojFiles pathToFolderWithSolutions

        pathsToCsprojFiles
        |> List.iter (fun csprojPath ->
            let patchedOutputPath = createOutputDirectoryForSolution csprojPath outputPath
            launchProcfiler csprojPath patchedOutputPath)
