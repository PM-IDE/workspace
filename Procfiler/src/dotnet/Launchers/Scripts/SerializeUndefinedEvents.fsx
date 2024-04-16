#load "../Core/Utils.fs"
#load "../Core/SerializeUndefinedThreadEvents.fs"

open Scripts.Core

let args = fsi.CommandLineArgs
SerializeUndefinedThreadEvents.launchProcfilerOnFolderOfSolutions args[1] args[2]
