namespace Scripts.Core

open System.IO
open Microsoft.FSharp.Core
open Microsoft.VisualBasic.CompilerServices
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
                    $" --use-during-runtime-filtering {this.DuringRuntimeFiltering}"]


    let private createConfigInternal csprojPath outputPath doInline merge onlineSerialization runtimeFiltering targetMethodsRegex =
        { Base = createDefaultConfigBase csprojPath outputPath
          Inline = doInline
          TargetMethodsRegex = targetMethodsRegex 
          FilterPattern = applicationNameFromCsproj csprojPath
          MergeUndefinedThreadEvents = merge
          OnlineSerialization = onlineSerialization
          DuringRuntimeFiltering = runtimeFiltering }

    let private createInlineMerge solutionPath outputPath : ICommandConfig =
        createConfigInternal solutionPath outputPath InlineMode.EventsAndMethodsEventsWithFilter true false false ".*"

    let private createNoInlineMerge solutionPath outputPath : ICommandConfig =
        createConfigInternal solutionPath outputPath InlineMode.NotInline true false true ".*"

    let private createInlineNoMerge solutionPath outputPath : ICommandConfig =
        createConfigInternal solutionPath outputPath InlineMode.EventsAndMethodsEventsWithFilter false true false ".*"

    let private createNoInlineNoMerge solutionPath outputPath : ICommandConfig =
        createConfigInternal solutionPath outputPath InlineMode.NotInline false true true ".*"

    let private allConfigs =
        [ ("inline_merge", createInlineMerge)
          ("no_inline_merge", createNoInlineMerge)
          ("no_inline_no_merge", createNoInlineNoMerge)
          ("inline_no_merge", createInlineNoMerge) ]

    let launchProcfiler csprojPath outputPath configFunc =
        ensureEmptyDirectory outputPath |> ignore
        launchProcfiler csprojPath outputPath configFunc

    let launchProcfilerOnFolderOfSolutions solutionsFolder outputPath =
        launchProcfilerOnFolderOfSolutions solutionsFolder outputPath createInlineMerge false

    let launchProcfilerOnSolutionsFolderInAllConfigs solutionsFolder outputPath =
        let pathsToDlls = getAllCsprojFiles solutionsFolder

        allConfigs
        |> List.iter (fun configInfo ->
            match configInfo with
            | configName, configFunc ->
                let outputPathForConfig = Path.Combine(outputPath, configName)
                ensureEmptyDirectory outputPathForConfig |> ignore

                pathsToDlls
                |> List.iter (fun solution -> launchProcfiler solution outputPathForConfig configFunc))
