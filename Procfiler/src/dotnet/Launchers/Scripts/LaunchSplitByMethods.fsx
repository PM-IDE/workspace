#load "../Core/Utils.fs"
#load "../Core/SplitByMethods.fs"

open Scripts.Core

let args = fsi.CommandLineArgs
SplitByMethods.launchProcfilerOnFolderOfSolutions args[1] args[2]
