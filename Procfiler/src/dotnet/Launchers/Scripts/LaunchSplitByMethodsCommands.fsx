#load "../Core/Utils.fs"
#load "../Core/SplitByMethods.fs"

open Scripts.Core

let args = fsi.CommandLineArgs
let commandsFile = args[1]
let outputFolder = args[2]

SplitByMethods.launchProcfilerOnCommands commandsFile outputFolder
