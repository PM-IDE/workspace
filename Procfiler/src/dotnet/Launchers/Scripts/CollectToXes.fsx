#load "../Core/Utils.fs"
#load "../Core/CollectToXes.fs"

open Scripts.Core

let args = fsi.CommandLineArgs
CollectToXes.launchProcfilerOnSolutionsFolder args[1] args[2]