#load "../Core/Utils.fs"
#load "../Core/SplitByMethods.fs"

open Scripts.Core

let args = fsi.CommandLineArgs
let commandsFile = args[0]
let outputFolder = args[1]

SplitByMethods.launchProcfilerOnCommands commandsFile outputFolder