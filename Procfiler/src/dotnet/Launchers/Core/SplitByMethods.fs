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


  let private createConfigInternal
    baseConfig
    doInline
    merge
    onlineSerialization
    runtimeFiltering
    targetMethodsRegex
    =
    { Base = baseConfig
      Inline = doInline
      TargetMethodsRegex = targetMethodsRegex
      FilterPattern = baseConfig.GetAppName()
      MergeUndefinedThreadEvents = merge
      OnlineSerialization = onlineSerialization
      DuringRuntimeFiltering = runtimeFiltering }

  let private createInlineMerge baseConfig : ICommandConfig =
    createConfigInternal baseConfig InlineMode.EventsAndMethodsEventsWithFilter true false false ".*"

  let private createNoInlineMerge baseConfig : ICommandConfig =
    createConfigInternal baseConfig InlineMode.NotInline true false true ".*"

  let private createInlineNoMerge baseConfig : ICommandConfig =
    createConfigInternal baseConfig InlineMode.EventsAndMethodsEventsWithFilter false true false ".*"

  let private createNoInlineNoMerge baseConfig : ICommandConfig =
    createConfigInternal baseConfig InlineMode.NotInline false true true ".*"

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
    { name: string
      command: string
      arguments: string }
  
  let launchProcfilerOnCommands commandsFile outputFolder =
    commandsFile
    |> File.ReadAllLines
    |> Array.map (fun line ->
        let parts = line.Split(';')
        {
          name = parts[0]
          command = parts[1]
          arguments = parts[2]
        })
    |> Array.iter (fun command ->
      let commandOutputFolder = Path.Combine(outputFolder, command.name)
      ensureEmptyDirectory commandOutputFolder |> ignore
      let baseConfig = createBaseCommandConfig command.command command.arguments commandOutputFolder
      let config = createInlineMerge baseConfig
      launchProcfiler config)
