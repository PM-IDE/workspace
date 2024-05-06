open System.Globalization
open System.IO
open System.Threading
open Bxes.Research.Core


[<EntryPoint>]
let main args =
    let logsTopLevelDirectory = args[0]
    let outputDirectory = args[1]

    Thread.CurrentThread.CurrentCulture <- CultureInfo("en-US")

    let processLogsDirectory (directory: string) =
        let currentOutputDirectory =
            Path.Combine(outputDirectory, Path.GetFileName(directory))

        Directory.GetFiles directory
        |> Seq.filter (fun dir -> dir.EndsWith(".xes"))
        |> Seq.map (fun logPath ->
            let logName = Path.GetFileNameWithoutExtension(logPath)
            let logOutputDirectory = Path.Combine(currentOutputDirectory, logName)

            (logName, Transformations.processEventLog logPath logOutputDirectory))
        |> Seq.toList

    match Directory.Exists(logsTopLevelDirectory) with
    | true ->
        use fs = File.OpenWrite(Path.Combine(outputDirectory, "results.csv"))
        use sw = new StreamWriter(fs)

        sw.WriteLine(
            "Name;OriginalSize;BxesSize;BxesPreprocessing;ZipSize;BxesToXesSize;ExiSize;ValuesRepeatCoef;AttributesRepeatCoef;ValuesVariantsCoef;AttributesVariantsCoef"
        )

        Directory.GetDirectories(logsTopLevelDirectory)
        |> Array.map (fun directory -> (directory, processLogsDirectory directory))
        |> Array.map (fun (directoryName, logsResults) ->
            logsResults
            |> List.map (fun (logName, logResults) ->
                let transformationResult =
                    logResults
                    |> List.map (fun res -> res.TransformedFileSize.ToString())
                    |> String.concat ";"

                let outputDir = logResults[0].TransformedFilesDirectory
                let valuesRepeatCoef = Transformations.extractValuesRepeatCoef outputDir
                let attributesRepeatCoef = Transformations.extractAttributesRepeatCoef outputDir

                let valuesVariantsCoef = Transformations.extractValuesVariantsCoef outputDir
                let attributesVariantsCoef = Transformations.extractAttributesVariantsCoef outputDir

                sw.WriteLine(
                    $"{logName};{logResults[0].OriginalFileSize};{transformationResult};{valuesRepeatCoef};{attributesRepeatCoef};{valuesVariantsCoef};{attributesVariantsCoef}"
                )))
        |> ignore
    | false -> printfn "The top level logs directory does not exist"

    0
