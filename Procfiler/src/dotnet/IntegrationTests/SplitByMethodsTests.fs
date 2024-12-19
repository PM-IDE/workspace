module IntegrationTests.SplitByMethodsTests

open System.IO
open NUnit.Framework
open Scripts.Core.ProcfilerScriptsUtils
open Scripts.Core.SplitByMethods
open Util

let private createConfigInternal csprojPath outputPath : ICommandConfig =
  { Config.Base = createBaseCsprojConfig csprojPath outputPath
    Config.Inline = InlineMode.EventsAndMethodsEventsWithFilter
    Config.FilterPattern = ".*"
    Config.TargetMethodsRegex = applicationNameFromCsproj csprojPath
    Config.MergeUndefinedThreadEvents = true
    Config.OnlineSerialization = true
    Config.DuringRuntimeFiltering = true }

let source () =
  seq {
    yield
      TestCaseData(
        "ConsoleApp1",
        [ "ConsoleApp1.Program..ctor[instance.void..()].xes"
          "ConsoleApp1.Program.Method2[void..()].xes"
          "ConsoleApp1.Program.Method1[void..()].xes"
          "ConsoleApp1.Program.Main[void..()].xes" ]
      )

    yield TestCaseData("SystemArrayPooling", [ "SystemArrayPooling.Program.Main[void..(class.System.String[])].xes" ])

    yield
      TestCaseData(
        "YieldEnumerator",
        [ "YieldEnumerator.Program+<<Main>g__EnumerateSomeNumbers|0_0>d.System.IDisposable.Dispose[instance.void..()].xes"
          "YieldEnumerator.Program+<<Main>g__EnumerateSomeNumbers|0_0>d..ctor[instance.void..(int32)].xes"
          "YieldEnumerator.Program+<<Main>g__EnumerateSomeNumbers|0_0>d.System.Collections.Generic.IEnumerable<System.Int32>.GetEnumerator[instance.class.System.Collections.Generic.IEnumerator`1<int32>..()].xes"
          "YieldEnumerator.Program.<Main>g__EnumerateSomeNumbers|0_0[class.System.Collections.Generic.IEnumerable`1<int32>..()].xes"
          "YieldEnumerator.Program+<<Main>g__EnumerateSomeNumbers|0_0>d.MoveNext[instance.bool..()].xes"
          "YieldEnumerator.Program.Main[void..(class.System.String[])].xes"
          "YieldEnumerator.Program+<<Main>g__EnumerateSomeNumbers|0_0>d.System.Collections.Generic.IEnumerator<System.Int32>.get_Current[instance.int32..()].xes"
          "YieldEnumerator.Program..ctor[instance.void..()].xes"
          "ASYNC_YieldEnumerator.Program+<<Main>g__EnumerateSomeNumbers|0_0>d.xes" ]
      )

    yield TestCaseData("UnsafeFixed", [ "UnsafeFixed.Program.Main[void..(class.System.String[])].xes" ])

    yield
      TestCaseData(
        "TaskTestProject1",
        [ "TaskTestProject1.Program+<>c__DisplayClass0_0.<Main>b__1[instance.void..(class.System.Threading.Tasks.Task`1<int32>)].xes"
          "TaskTestProject1.Program.Main[void..(class.System.String[])].xes"
          "TaskTestProject1.Program+<>c__DisplayClass0_0..ctor[instance.void..()].xes"
          "TaskTestProject1.Program+<>c__DisplayClass0_0.<Main>b__0[instance.int32..()].xes" ]
      )

    yield
      TestCaseData(
        "SimpleAsyncAwait",
        [ "SimpleAsyncAwait.Program+<Main>d__0.MoveNext[instance.void..()].xes"
          "SimpleAsyncAwait.Program+<Main>d__0..ctor[instance.void..()].xes"
          "SimpleAsyncAwait.Class3..ctor[instance.void..()].xes"
          "SimpleAsyncAwait.Program.<Main>[void..(class.System.String[])].xes"
          "ASYNC_SimpleAsyncAwait.Program+<<Main>g__Foo|0_1>d.xes"
          "SimpleAsyncAwait.Program+<<Main>g__Foo|0_1>d.MoveNext[instance.void..()].xes"
          "SimpleAsyncAwait.Program.Main[class.System.Threading.Tasks.Task..(class.System.String[])].xes"
          "SimpleAsyncAwait.Program+<>c..ctor[instance.void..()].xes"
          "SimpleAsyncAwait.Program+<>c..cctor[void..()].xes"
          "SimpleAsyncAwait.Program.<Main>g__Foo|0_1[class.System.Threading.Tasks.Task`1<int32>..()].xes"
          "SimpleAsyncAwait.Program+<>c.<Main>b__0_0[instance.int32..()].xes"
          "SimpleAsyncAwait.Class2..ctor[instance.void..()].xes"
          "ASYNC_SimpleAsyncAwait.Program+<Main>d__0.xes"
          "SimpleAsyncAwait.Program+<<Main>g__Foo|0_1>d..ctor[instance.void..()].xes"
          "SimpleAsyncAwait.Class1..ctor[instance.void..()].xes" ]
      )

    yield
      TestCaseData(
        "NotSimpleAsyncAwait",
        [ "NotSimpleAsyncAwait.Class2..ctor[instance.void..()].xes"
          "NotSimpleAsyncAwait.Program+<>c__DisplayClass0_0.<Main>b__1[instance.class.System.Threading.Tasks.Task..()].xes"
          "NotSimpleAsyncAwait.Class1..ctor[instance.void..()].xes"
          "ASYNC_NotSimpleAsyncAwait.Program+<>c__DisplayClass0_0+<<Main>b__1>d.xes"
          "NotSimpleAsyncAwait.Class4..ctor[instance.void..()].xes"
          "NotSimpleAsyncAwait.Program+<>c__DisplayClass0_0+<<Main>b__1>d..ctor[instance.void..()].xes"
          "ASYNC_NotSimpleAsyncAwait.Program+<Main>d__0.xes"
          "NotSimpleAsyncAwait.Program+<Main>d__0.MoveNext[instance.void..()].xes"
          "NotSimpleAsyncAwait.Program.<Main>[void..(class.System.String[])].xes"
          "NotSimpleAsyncAwait.Program.Main[class.System.Threading.Tasks.Task..(class.System.String[])].xes"
          "NotSimpleAsyncAwait.Program+<Main>d__0..ctor[instance.void..()].xes"
          "NotSimpleAsyncAwait.Program.<Main>g__Allocate|0_0[class.System.Object..(int32)].xes"
          "NotSimpleAsyncAwait.Program+<>c__DisplayClass0_0..ctor[instance.void..()].xes"
          "NotSimpleAsyncAwait.Class3..ctor[instance.void..()].xes"
          "NotSimpleAsyncAwait.Program+<>c__DisplayClass0_0+<<Main>b__1>d.MoveNext[instance.void..()].xes" ]
      )

    yield
      TestCaseData(
        "NotExistingAssemblyLoading",
        [ "NotExistingAssemblyLoading.Program.Main[void..(class.System.String[])].xes"
          "NotExistingAssemblyLoading.Program+<>c..cctor[void..()].xes"
          "NotExistingAssemblyLoading.Program+<>c.<Main>b__0_0[instance.class.System.Reflection.Assembly..(class.System.Object,class.System.ResolveEventArgs)].xes"
          "NotExistingAssemblyLoading.Program+<>c..ctor[instance.void..()].xes"
          "NotExistingAssemblyLoading.Program+<>c.<Main>b__0_1[instance.class.System.Reflection.Assembly..(class.System.Runtime.Loader.AssemblyLoadContext,class.System.Reflection.AssemblyName)].xes" ]
      )

    yield TestCaseData("LOHAllocations", [ "LOHAllocations.Program.Main[void..(class.System.String[])].xes" ])

    yield
      TestCaseData(
        "IntensiveThreadPoolUse",
        [ "IntensiveThreadPoolUse.Program.Main[void..(class.System.String[])].xes" ]
      )

    yield
      TestCaseData(
        "FinalizableObject",
        [ "FinalizableObject.Program.Main[void..(class.System.String[])].xes"
          "FinalizableObject.ClassWithFinalizer.Finalize[instance.void..()].xes"
          "FinalizableObject.ClassWithFinalizer..ctor[instance.void..()].xes" ]
      )

    yield
      TestCaseData(
        "FileWriteProject",
        [ "FileWriteProject.Program.<Main>[void..(class.System.String[])].xes"
          "FileWriteProject.Program.Main[class.System.Threading.Tasks.Task..(class.System.String[])].xes"
          "ASYNC_FileWriteProject.Program+<Main>d__0.xes"
          "FileWriteProject.Program+<Main>d__0.MoveNext[instance.void..()].xes"
          "FileWriteProject.Program+<Main>d__0..ctor[instance.void..()].xes" ]
      )

    yield
      TestCaseData(
        "FileAsyncOperations",
        [ "FileAsyncOperations.Class3..ctor[instance.void..()].xes"
          "FileAsyncOperations.Program+<Main>d__0..ctor[instance.void..()].xes"
          "FileAsyncOperations.Program+<Main>d__0.MoveNext[instance.void..()].xes"
          "FileAsyncOperations.Program.Main[class.System.Threading.Tasks.Task..(class.System.String[])].xes"
          "FileAsyncOperations.Program.<Main>[void..(class.System.String[])].xes"
          "FileAsyncOperations.Class1..ctor[instance.void..()].xes"
          "FileAsyncOperations.Class2..ctor[instance.void..()].xes"
          "ASYNC_FileAsyncOperations.Program+<Main>d__0.xes" ]
      )

    yield
      TestCaseData(
        "ExceptionTryCatchFinallyWhen",
        [ "ExceptionTryCatchFinallyWhen.Program.<Main>g__Throw|0_0[void..()].xes"
          "ExceptionTryCatchFinallyWhen.Program.Main[void..(class.System.String[])].xes" ]
      )

    yield
      TestCaseData(
        "ExceptionTryCatchFinallyAsync",
        [ "ExceptionTryCatchFinallyAsync.Program.Main[class.System.Threading.Tasks.Task..(class.System.String[])].xes"
          "ExceptionTryCatchFinallyAsync.Program.<Main>g__Print|0_2[class.System.Threading.Tasks.Task..()].xes"
          "ExceptionTryCatchFinallyAsync.Program.<Main>[void..(class.System.String[])].xes"
          "ExceptionTryCatchFinallyAsync.Program+<>c.<Main>b__0_0[instance.class.System.Threading.Tasks.Task..(int32)].xes"
          "ExceptionTryCatchFinallyAsync.Program+<>c..ctor[instance.void..()].xes"
          "ExceptionTryCatchFinallyAsync.Program+<<Main>g__Print|0_2>d..ctor[instance.void..()].xes"
          "ExceptionTryCatchFinallyAsync.Program+<<Main>g__Print|0_2>d.MoveNext[instance.void..()].xes"
          "ExceptionTryCatchFinallyAsync.Program+<Main>d__0.MoveNext[instance.void..()].xes"
          "ASYNC_ExceptionTryCatchFinallyAsync.Program+<<Main>g__X|0_1>d.xes"
          "ExceptionTryCatchFinallyAsync.Program+<>c__DisplayClass0_0.<Main>b__3[instance.void..()].xes"
          "ExceptionTryCatchFinallyAsync.Program+<>c..cctor[void..()].xes"
          "ASYNC_ExceptionTryCatchFinallyAsync.Program+<Main>d__0.xes"
          "ExceptionTryCatchFinallyAsync.Program+<<Main>g__X|0_1>d..ctor[instance.void..()].xes"
          "ExceptionTryCatchFinallyAsync.Program.<Main>g__X|0_1[class.System.Threading.Tasks.Task..(int32)].xes"
          "ExceptionTryCatchFinallyAsync.Program+<>c__DisplayClass0_0..ctor[instance.void..()].xes"
          "ASYNC_ExceptionTryCatchFinallyAsync.Program+<<Main>g__Print|0_2>d.xes"
          "ExceptionTryCatchFinallyAsync.Program+<Main>d__0..ctor[instance.void..()].xes"
          "ExceptionTryCatchFinallyAsync.Program+<<Main>g__X|0_1>d.MoveNext[instance.void..()].xes" ]
      )

    yield
      TestCaseData(
        "ExceptionTryCatchFinally",
        [ "ExceptionTryCatchFinally.Program..ctor[instance.void..()].xes"
          "ExceptionTryCatchFinally.Program.Main[void..(class.System.String[])].xes"
          "ExceptionTryCatchFinally.Program.<Main>g__ThrowException|0_0[void..()].xes" ]
      )

    yield
      TestCaseData(
        "DynamicAssemblyLoading",
        [ "DynamicAssemblyLoading.Program.Main[void..(class.System.String[])].xes" ]
      )

    yield
      TestCaseData(
        "DynamicAssemblyCreation",
        [ "DynamicAssemblyCreation.Program.Main[void..(class.System.String[])].xes" ]
      )
  }

[<TestCaseSource("source")>]
let SplitByMethodsTest projectName (expectedMethods: List<string>) =
  let doTest tempDir =
    let path = getCsprojPathFromSource projectName
    launchProcfiler (createConfigInternal path tempDir)

    let fileNameProcessor (name: string) =
      let chars =
        name.ToLower().ToCharArray()
        |> Array.filter (fun c -> (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9'))

      new string (chars)

    let files =
      Directory.GetFiles(tempDir)
      |> Array.map Path.GetFileName
      |> Array.map fileNameProcessor
      |> Set.ofArray

    let expectedFileNames = expectedMethods |> List.map fileNameProcessor |> Set.ofList
    Assert.That(Set.isSubset expectedFileNames files, Is.True)

  executeTestWithTempFolder doTest
