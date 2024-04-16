namespace Scripts.Core

open Scripts.Core.ProcfilerScriptsUtils

module SplitByNames =
    type Config =
        { PathConfig: PathConfigBase }

        interface ICommandConfig with
            member this.CreateArguments() =
                [ "split-by-names" ] |> this.PathConfig.AddArguments


    let private createConfig csprojPath outputPath =
        { PathConfig =
            { CsprojPath = csprojPath
              OutputPath = outputPath } }

    let launchProcfilerCustomConfig csprojPath outputPath createConfig =
        launchProcfiler csprojPath outputPath createConfig

    let launchProcfiler csprojPath outputPath =
        launchProcfilerCustomConfig csprojPath outputPath
