namespace Scripts.Core

open System.IO
open Microsoft.FSharp.Core
open Scripts.Core.ProcfilerScriptsUtils

module SplitByMethods =
  type InlineMode =
    | NotInline
    | OnlyEvents
    | EventsAndMethodsEvents
    | EventsAndMethodsEventsWithFilter

  type Config =
    { Base: ConfigBase
      Inline: InlineMode
      FilterPattern: string
      TargetMethodsRegex: string
      MergeUndefinedThreadEvents: bool
      OnlineSerialization: bool
      DuringRuntimeFiltering: bool }

    interface ICommandConfig with
      member this.CreateArguments() =
        let args = [ "split-by-methods" ]
        let onlineMode = "PerThreadBinStacksFilesOnline"
        let offlineMode = "SingleFileBinStack"

        this.Base.AddArguments args
        @ [ $" --methods-filter-regex {this.FilterPattern}"
            $" --target-methods-regex {this.TargetMethodsRegex}"
            $" --inline {this.Inline}"
            $" --merge-undefined-events {this.MergeUndefinedThreadEvents}"
            $" --cpp-profiler-mode {if this.OnlineSerialization then onlineMode else offlineMode}"
            $" --use-during-runtime-filtering {this.DuringRuntimeFiltering}" ]
        
      member this.GetAppName() = this.Base.GetAppName()
      member this.GetWorkingDirectory() = this.Base.GetWorkingDirectory()
      member this.GetFilterPattern() = this.Base.GetFilterPattern()


  let private createConfigInternal
    baseConfig
    doInline
    merge
    onlineSerialization
    runtimeFiltering
    =
    { Base = baseConfig
      Inline = doInline
      TargetMethodsRegex = baseConfig.GetFilterPattern()
      FilterPattern = ".*"
      MergeUndefinedThreadEvents = merge
      OnlineSerialization = onlineSerialization
      DuringRuntimeFiltering = runtimeFiltering }

  let private createInlineMerge baseConfig : ICommandConfig =
    createConfigInternal baseConfig InlineMode.EventsAndMethodsEvents true false false 

  let private createNoInlineMerge baseConfig : ICommandConfig =
    createConfigInternal baseConfig InlineMode.NotInline true false true

  let private createInlineNoMerge baseConfig : ICommandConfig =
    createConfigInternal baseConfig InlineMode.EventsAndMethodsEvents false true false

  let private createNoInlineNoMerge baseConfig : ICommandConfig =
    createConfigInternal baseConfig InlineMode.NotInline false true true

  let private allConfigs =
    [ ("inline_merge", createInlineMerge)
      ("no_inline_merge", createNoInlineMerge)
      ("no_inline_no_merge", createNoInlineNoMerge)
      ("inline_no_merge", createInlineNoMerge) ]

  let launchProcfilerOnFolderOfSolutions solutionsFolder outputPath =
    launchProcfilerOnFolderOfSolutions solutionsFolder outputPath createInlineMerge false

  let launchProcfilerOnSolutionsFolderInAllConfigs solutionsFolder outputPath =
    let pathsToCsprojes = getAllCsprojFiles solutionsFolder

    allConfigs
    |> List.iter (fun (configName, configFunc) ->
                    let outputPathForConfig = Path.Combine(outputPath, configName)
                    ensureEmptyDirectory outputPathForConfig |> ignore

                    pathsToCsprojes 
                    |> List.iter (fun csprojPath ->
                                    let baseConfig = createBaseCsprojConfig csprojPath outputPathForConfig
                                    let config = configFunc baseConfig
                                    ensureEmptyDirectory outputPath |> ignore
                                    launchProcfiler config))
  
  type CommandToExecute =
    { Name: string
      Command: string
      Arguments: string
      FilterPattern: string }
  
  let launchProcfilerOnCommands commandsFile outputFolder =
    commandsFile
    |> File.ReadAllLines
    |> Array.map (fun line ->
        let parts = line.Split(';')
        {
          Name = parts[0]
          Command = parts[1]
          Arguments = parts[2]
          FilterPattern = parts[3]
        })
    |> Array.iter (fun command ->
      let commandOutputFolder = Path.Combine(outputFolder, command.Name)
      ensureEmptyDirectory commandOutputFolder |> ignore
      let baseConfig = createBaseCommandConfig command.Command command.Arguments command.FilterPattern commandOutputFolder
      let config = createInlineMerge baseConfig
      launchProcfiler config)
