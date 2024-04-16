namespace Bxes.Research.Core

open System
open System.Collections.Generic
open System.Diagnostics
open System.IO
open System.IO.Compression
open Bxes.Utils
open Bxes.Xes.BxesToXes
open Bxes.Xes.XesToBxes

module Transformations =
    type TransformationResult =
        { TransformationName: string
          OriginalFilePath: string
          TransformedFilesDirectory: string
          OriginalFileSize: int64
          TransformedFileSize: int64 }

    let createOutputFilePath (xesFilePath: string) outputDirectory extension =
        let bxesLogFileName = Path.GetFileNameWithoutExtension(xesFilePath) + extension
        Path.Combine(outputDirectory, bxesLogFileName)

    let executeTransformation logPath outputDirectory transformationName extension transformation =
        let outputPath = createOutputFilePath logPath outputDirectory extension
        let outputDirectory = Path.GetDirectoryName outputPath

        match Directory.Exists outputDirectory with
        | true -> ()
        | false ->
            Directory.CreateDirectory outputDirectory |> ignore
            ()

        printfn $"Started executing transformation {transformationName} for file {logPath}"
        transformation outputPath

        { TransformationName = extension
          OriginalFilePath = logPath
          TransformedFilesDirectory = outputDirectory 
          OriginalFileSize = FileInfo(logPath).Length
          TransformedFileSize = FileInfo(outputPath).Length }

    let private bxesTransformationBase logPath outputDirectory preprocess =
        let name = if preprocess then "BxesPreprocessing" else "Bxes"
        let extension = if preprocess then ".bxespreprocessing" else ".bxes"

        executeTransformation logPath outputDirectory name extension (fun outputPath ->
            let logger = BxesDefaultLoggerFactory.Create()
            XesToBxesConverter(logger, preprocess, true).Convert(logPath, outputPath))

    let bxesPreprocessingTransformation logPath outputDirectory =
        bxesTransformationBase logPath outputDirectory true

    let bxesDefaultTransformation logPath outputDirectory =
        bxesTransformationBase logPath outputDirectory false

    let zipTransformation logPath outputDirectory =
        executeTransformation logPath outputDirectory "Zip" ".zip" (fun outputPath ->
            use fs = File.OpenWrite(outputPath)
            use archive = new ZipArchive(fs, ZipArchiveMode.Create)

            let fileName = Path.GetFileNameWithoutExtension(logPath)

            archive.CreateEntryFromFile(logPath, fileName, CompressionLevel.SmallestSize)
            |> ignore

            ())

    let bxesToXesTransformation (logPath: string) outputDirectory =
        let mutable bxesOutputPath = ""

        executeTransformation logPath outputDirectory "Bxes" ".bxes" (fun outputPath -> bxesOutputPath <- outputPath)
        |> ignore

        executeTransformation logPath outputDirectory "BxesToXes" ".xes" (fun outputPath ->
            BxesToXesConverter().Convert(bxesOutputPath, outputPath))

    let private executeExternalTransformation command (arguments: string list) =
        let transformationProcess = Process.Start(command, arguments)
        transformationProcess.WaitForExit()

        match transformationProcess.ExitCode with
        | 0 -> ()
        | _ -> raise (Exception())

    let private executeExternalJavaTransformation jarPath logPath outputFilePath =
        executeExternalTransformation "java" [ "-jar"; jarPath; logPath; outputFilePath ]

    let exiTransformation (logPath: string) outputDirectory =
        executeTransformation logPath outputDirectory "EXI" ".exi" (fun outputPath ->
            let exiTransformerJarPath = Environment.GetEnvironmentVariable("EXI_JAR_PATH")
            executeExternalJavaTransformation exiTransformerJarPath logPath outputPath)

    let transformations =
        [ bxesDefaultTransformation
          bxesPreprocessingTransformation
          zipTransformation
          bxesToXesTransformation
          exiTransformation ]

    let processEventLog logPath outputDirectory =
        transformations
        |> List.map (fun transformation -> transformation logPath outputDirectory)

    let private calculateRepeatCoef (data: Dictionary<'a, int>) =
        let max = double (data.Values |> Seq.max)
        let min = double (data.Values |> Seq.min)
        (data.Values |> Seq.map (fun x -> (double x - min) / (max - min)) |> Seq.sum) / double data.Count

    let private calculateVariantsCoef (data: Dictionary<'a, int>) =
        double data.Count / double (data.Values |> Seq.sum)
        
    let private doExtractValuesRepeatCoef outputDirectory coefFunc =
        let valuesStatPath = Path.Combine(outputDirectory, XesToBxesStatisticFiles.ValuesStatistics)
        let stat = XesToBxesStatisticUtil.ReadValuesStatistics valuesStatPath
        coefFunc stat
        
    let private doExtractAttributesRepeatCoef outputDirectory coefFunc =
        let valuesStatPath = Path.Combine(outputDirectory, XesToBxesStatisticFiles.AttributesStatistics)
        let stat = XesToBxesStatisticUtil.ReadAttributesStatistics valuesStatPath
        coefFunc stat

    let extractValuesRepeatCoef outputDirectory =
        doExtractValuesRepeatCoef outputDirectory calculateRepeatCoef

    let extractAttributesRepeatCoef outputDirectory =
        doExtractAttributesRepeatCoef outputDirectory calculateRepeatCoef

    let extractValuesVariantsCoef outputDirectory =
        doExtractValuesRepeatCoef outputDirectory calculateVariantsCoef

    let extractAttributesVariantsCoef outputDirectory =
        doExtractAttributesRepeatCoef outputDirectory calculateVariantsCoef