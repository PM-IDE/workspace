namespace Scripts.Core

open Scripts.Core.ProcfilerScriptsUtils

module SerializeUndefinedThreadEvents =
  type Config =
    { Base: ConfigBase }

    interface ICommandConfig with
      member this.CreateArguments() = [ "undefined-events-to-xes" ] |> this.Base.AddArguments
      member this.GetAppName() = this.Base.GetAppName()
      member this.GetWorkingDirectory() = this.Base.GetWorkingDirectory()


  let private createConfig csprojPath outputPath : ICommandConfig =
    { Base = createBaseCsprojConfig csprojPath outputPath }

  let launchProcfilerCustomConfig csprojPath outputPath createCustomConfig =
    ensureEmptyDirectory outputPath |> ignore
    launchProcfiler (createCustomConfig csprojPath outputPath)

  let launchProcfiler csprojPath outputPath =
    launchProcfilerCustomConfig csprojPath outputPath createConfig

  let launchProcfilerOnFolderOfSolutions pathToFolderWithSolutions outputPath =
    let pathsToCsprojFiles = getAllCsprojFiles pathToFolderWithSolutions

    pathsToCsprojFiles
    |> List.iter (fun csprojPath ->
      let patchedOutputPath = createOutputDirectoryForSolution csprojPath outputPath
      launchProcfiler csprojPath patchedOutputPath)
