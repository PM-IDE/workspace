open System.IO
open Microsoft.FSharp.Core

let renameToGoldIfTmp (filePath: string) =
    match Path.GetExtension(filePath) with
    | ".tmp" ->
        let name = Path.GetFileNameWithoutExtension(filePath)
        let directory = Path.GetDirectoryName(filePath)
        let goldFilePath = Path.Combine(directory, $"{name}")
        File.Move(filePath, goldFilePath, true)
        printfn $"Renamed {filePath} to {goldFilePath}"
    | _ -> ()

let rec tmpToGoldRecursive folder =
    Directory.GetFiles(folder) |> Seq.iter renameToGoldIfTmp
    Directory.EnumerateDirectories(folder) |> Seq.iter tmpToGoldRecursive

tmpToGoldRecursive fsi.CommandLineArgs[1]
