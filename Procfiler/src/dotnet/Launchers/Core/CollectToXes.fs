namespace Scripts.Core

open Scripts.Core.ProcfilerScriptsUtils

module CollectToXes =
  type Config =
    { Base: ConfigBase }

    interface ICommandConfig with
      member this.CreateArguments() =
        [ "collect-to-xes" ] |> this.Base.AddArguments


  let private createConfig csprojPath outputPath : ICommandConfig =
    { Base = createDefaultConfigBase csprojPath outputPath }

  let launchProcfilerCustomConfig csprojPath outputPath createConfig =
    launchProcfiler csprojPath outputPath createConfig

  let launchProcfiler csprojPath outputPath =
    launchProcfilerCustomConfig csprojPath outputPath

  let launchProcfilerOnSolutionsFolder solutionsFolder outputFolder =
    launchProcfilerOnFolderOfSolutions solutionsFolder outputFolder createConfig true
