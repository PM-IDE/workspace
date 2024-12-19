namespace Scripts.Core

open System.Text
open System
open System.Diagnostics
open System.IO


module ProcfilerScriptsUtils =
  let net7 = "net7.0"
  let net6 = "net6.0"
  let net8 = "net8.0"

  type CsprojRequiredArguments =
    { CsprojPath: string }

    member this.AddArguments list =
      list @ [ $" -csproj {this.CsprojPath}"; ]

  
  type CommandRequiredArguments =
    { Command: string
      Arguments: string }
    
    member this.AddArguments list =
      list @ [ $" -command {this.Command}"; $" --arguments {this.Arguments}" ]
      

  type RequiredArguments =
    | Command of CommandRequiredArguments
    | Csproj of CsprojRequiredArguments
    
    member this.AddArguments list =
      match this with
      | Csproj csproj -> csproj.AddArguments list
      | Command command -> command.AddArguments list

  type ICommandConfig =
    abstract member CreateArguments: unit -> string list
    abstract member GetWorkingDirectory: unit -> string
    abstract member GetAppName: unit -> string

  let applicationNameFromCsproj (dllPath: string) =
    let csprojName = Path.GetFileName(dllPath)
    csprojName.AsSpan().Slice(0, csprojName.IndexOf('.')).ToString()
    
  type ConfigBase =
    { RequiredArgs: RequiredArguments
      Duration: int
      Repeat: int
      WriteAllMetadata: bool
      OutputPath: string }

    member this.AddArguments list =
      let toAdd =
        [ $" --repeat {this.Repeat}"
          $" --duration {this.Duration}"
          $" --write-all-event-metadata {this.WriteAllMetadata}"
          $" -o {this.OutputPath}"
          " --log-serialization-format bxes" ]

      this.RequiredArgs.AddArguments list @ toAdd
      
    member this.GetWorkingDirectory() =
      match this.RequiredArgs with
      | Csproj csproj -> Path.GetDirectoryName csproj.CsprojPath
      | Command _ -> Directory.GetCurrentDirectory()
      
    member this.GetAppName() =
      match this.RequiredArgs with
      | Csproj csproj -> applicationNameFromCsproj csproj.CsprojPath
      | Command command -> command.Command
        
  let createBaseCsprojConfig csprojPath outputPath =
    { RequiredArgs = Csproj({
        CsprojPath = csprojPath
      })

      OutputPath = outputPath
      Duration = 10_000
      Repeat = 20
      WriteAllMetadata = false }
    
  let createBaseCommandConfig command arguments outputPath =
    { RequiredArgs = Command({
        Command = command
        Arguments = arguments
      })

      OutputPath = outputPath
      Duration = 10_000
      Repeat = 20
      WriteAllMetadata = false }

  let private createProcess fileName (args: String) workingDirectory =
    let startInfo = ProcessStartInfo(fileName, args)
    startInfo.WorkingDirectory <- workingDirectory
    new Process(StartInfo = startInfo)

  let buildProjectFromSolution solutionDirectory projectName =
    let projectPath = $"./{projectName}/{projectName}.csproj"
    let pRelease = "/p:Configuration=\"Release\""

    let pSolutionDir =
      $"/p:SolutionDir={solutionDirectory}{Path.DirectorySeparatorChar}"

    let args = $"msbuild {projectPath} {pRelease} {pSolutionDir}"
    let buildProcess = createProcess "dotnet" args solutionDirectory

    match buildProcess.Start() with
    | false -> printfn $"Build process for solution {solutionDirectory} failed to start"
    | true ->
      buildProcess.WaitForExit()

      match buildProcess.ExitCode with
      | 0 -> printfn $"Successfully built {solutionDirectory}/{projectName}"
      | _ -> printfn $"Error happened when building solution {solutionDirectory}/{projectName}:"


  let rec private findProperParentDirectory (currentDirectory: string) =
    let name = Path.GetFileName currentDirectory

    match name with
    | "src" -> currentDirectory
    | _ -> findProperParentDirectory (currentDirectory |> Directory.GetParent).FullName

  let buildProcfiler =
    let parentDirectory = (Directory.GetCurrentDirectory() |> Directory.GetParent).FullName

    let dir = findProperParentDirectory parentDirectory
    let dotnetSourcePath = Path.Combine(dir, "dotnet")

    let framework = net8

    printfn "Started building ProcfilerBuildTasks"
    buildProjectFromSolution dotnetSourcePath "ProcfilerBuildTasks"

    printfn "Started building whole Procfiler solution"
    buildProjectFromSolution dotnetSourcePath "Procfiler"

    Path.Combine(dotnetSourcePath, "Procfiler", "bin", "Release", framework, "Procfiler.dll")

  let getAllCsprojFiles solutionsDirectory =
    Directory.GetDirectories(solutionsDirectory)
    |> List.ofArray
    |> List.map (fun dir -> Path.Combine(dir, Path.GetFileName(dir) + ".csproj"))

  let ensureEmptyDirectory path =
    match Directory.Exists path with
    | true ->
      Directory.Delete(path, true)
      Directory.CreateDirectory path
    | false -> Directory.CreateDirectory path

  let getAllSolutionsFrom directory =
    directory |> Directory.GetDirectories |> List.ofArray

  let createOutputDirectoryForSolution csprojPath outputFolder =
    let appName = applicationNameFromCsproj csprojPath
    let outputPathForSolution = Path.Combine(outputFolder, appName)
    ensureEmptyDirectory outputPathForSolution |> ignore
    outputPathForSolution

  let createArgumentsString (config: ICommandConfig) =
    let sb = StringBuilder()

    config.CreateArguments()
    |> List.iter (fun (arg: string) -> sb.Append arg |> ignore)

    sb.ToString()
    
  let launchProcfiler config =
    let args = createArgumentsString config
    let workingDirectory = config.GetWorkingDirectory()
    let procfilerProcess = createProcess "dotnet" $"{buildProcfiler} {args}" workingDirectory

    match procfilerProcess.Start() with
    | true ->
      printfn $"Started procfiler for {config.GetAppName()}"
    | false -> printfn "Failed to start procfiler"

    procfilerProcess.WaitForExit()

    match procfilerProcess.ExitCode with
    | 0 ->
      printfn $"Finished executing procfiler for {config.GetAppName()}"
    | _ -> ()

  let launchProcfilerOnFolderOfSolutions solutionsFolder outputFolder baseConfigCreator outputIsFile =
    ensureEmptyDirectory outputFolder |> ignore
    let pathsToCsprojes = getAllCsprojFiles solutionsFolder

    pathsToCsprojes
    |> List.iter (fun csprojPath ->
      let name = applicationNameFromCsproj csprojPath

      let outputPath =
        match outputIsFile with
        | true -> Path.Combine(outputFolder, name + ".xes")
        | false ->
          let directory = Path.Combine(outputFolder, name)
          Directory.CreateDirectory(directory) |> ignore
          directory

      let config =  createBaseCsprojConfig csprojPath outputPath
      launchProcfiler (baseConfigCreator config))
