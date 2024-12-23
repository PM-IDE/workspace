namespace Scripts.Core

open Scripts.Core.ProcfilerScriptsUtils

module SplitByNames =
  type Config =
    { Base: ConfigBase }

    interface ICommandConfig with
      member this.CreateArguments() = [ "split-by-names" ] |> this.Base.AddArguments  
      member this.GetAppName() = this.Base.GetAppName()
      member this.GetWorkingDirectory() = this.Base.GetWorkingDirectory()
      member this.GetFilterPattern() = this.Base.GetFilterPattern()


  let createConfig csprojPath outputPath =
    { Base = createBaseCsprojConfig csprojPath outputPath }

  let launchProcfilerCustomConfig csprojPath outputPath createConfig =
    launchProcfiler (createConfig csprojPath outputPath) 

  let launchProcfiler csprojPath outputPath =
    launchProcfilerCustomConfig csprojPath outputPath
