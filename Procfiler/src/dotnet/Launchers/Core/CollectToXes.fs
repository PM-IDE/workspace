namespace Scripts.Core

open Scripts.Core.ProcfilerScriptsUtils

module CollectToXes =
  type Config =
    { Base: ConfigBase }

    interface ICommandConfig with
      member this.CreateArguments() = [ "collect-to-xes" ] |> this.Base.AddArguments
      member this.GetAppName() = this.Base.GetAppName()
      member this.GetWorkingDirectory() = this.Base.GetWorkingDirectory()


  let private createConfig csprojPath outputPath : ICommandConfig =
    { Base = createBaseCsprojConfig csprojPath outputPath }

  let launchProcfilerCustomConfig csprojPath outputPath createConfig =
    launchProcfiler (createConfig csprojPath outputPath)

  let launchProcfiler csprojPath outputPath =
    launchProcfilerCustomConfig csprojPath outputPath

  let launchProcfilerOnSolutionsFolder solutionsFolder outputFolder =
    let createConfig baseConfig = {
      Base = baseConfig
    }

    launchProcfilerOnFolderOfSolutions solutionsFolder outputFolder createConfig true

