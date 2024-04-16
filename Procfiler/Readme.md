# Procfiler

Procfiler is a tool which aims to bridge the gap between .NET and Process Mining. The tool supports collecting CLR (ETW)
events through EventPipe, supports various process instances (method invocation, whole program execution) and can
serialize
obtained event logs into different formats, in particular into XES. XES event logs can be then analyzed with different
Process Mining tools, such as `ProM` or `pm4py`.

## Sources

- https://github.com/ABaboshin/DotNetCoreProfiler
- https://github.com/dotnet/runtime/tree/main/src/tests/profiler
- https://github.com/dotnet/runtime/tree/main/docs/design/coreclr/profiling
- https://github.com/Microsoft/perfview

Procfiler is a console application which supports following commands:

## Top level commands:

Usage:
procfiler [command] [options]

Options:

```
--version       Show version information
-?, -h, --help  Show help and usage information
```

Commands:

```
create-documentation     Generate documentation for filters, mutators and other application components  
collect-to-xes           Collect CLR events and serialize them to XES format
meta-info                Collects meta-information about CLR events received during listening to process
undefined-events-to-xes  Collect CLR events from undefined thread and serialize them to XES format      
split-by-names           Split the events into different files based on managed thread ID
split-by-threads         Split the events into different files based on managed thread ID
split-by-methods         Splits the events by methods, in which they occured, and serializes to XES 
```

## create-documentation

Usage:

```
procfiler create-documentation [options]
```

Options:

```
-o <o> (REQUIRED)  The path to the directory into which the documentation will be generated
-?, -h, --help     Show help and usage information
```

## collect-to-xes

Usage:

```
procfiler collect-to-xes [options]
```

Options:

```
--repeat <repeat>                                           The number of times launching of the program should be repeated [default: 1]
-p <p>                                                      The process id from which we should collect CLR events
-csproj <csproj>                                            The path to the .csproj file of the project to be executed
-o <o> (REQUIRED)                                           The output path
--duration <duration>                                       The amount of time to spend collecting CLR events [default: 60000]
--format <Csv|MethodCallTree>                               The output file(s) format [default: Csv]
--timeout <timeout>                                         The timeout which we want to wait until processing all events [default: 10000]
--clear-before                                              Clear (delete) output folder (file) before profiling session [default: True]
--merge-undefined-events                                    Should we merge events from undefined thread to managed thread events [default: True]
--tfm <tfm>                                                 The target framework identifier, the project will be built for specified tfm [default: net6.0]
--c <Debug|Release>                                         Build configuration which will be used during project build [default: Debug]
--providers <All|Gc|GcAllocHigh|GcAllocLow>                 Providers which will be used for collecting events [default: All]
--instrument <MainAssembly|MainAssemblyAndReferences|None>  Kind of instrumentation to be used [default: None]
--self-contained                                            Whether to build application in a self-contained mode [default: False]
--temp <temp>                                               Folder which will be used for temp artifacts of events collection []
--remove-temp                                               Whether to remove temp directory for artifacts after finishing work [default: True]
--arguments <arguments>                                     Arguments which will be passed when launching the program []
--arguments-file <arguments-file>                           File containing list of arguments which will be passed to program []
--print-process-output                                      Whether to print the output of a profiled application [default: True]
--filter <filter>                                           Regex to filter methods []
-?, -h, --help                                              Show help and usage information
```

## undefined-events-to-xes

Usage:

```
procfiler undefined-events-to-xes [options]
```

Options:

```
--repeat <repeat>                                           The number of times launching of the program should be repeated [default: 1]
-p <p>                                                      The process id from which we should collect CLR events
-csproj <csproj>                                            The path to the .csproj file of the project to be executed
-o <o> (REQUIRED)                                           The output path
--duration <duration>                                       The amount of time to spend collecting CLR events [default: 60000]
--format <Csv|MethodCallTree>                               The output file(s) format [default: Csv]
--timeout <timeout>                                         The timeout which we want to wait until processing all events [default: 10000]
--clear-before                                              Clear (delete) output folder (file) before profiling session [default: True]
--merge-undefined-events                                    Should we merge events from undefined thread to managed thread events [default: True]
--tfm <tfm>                                                 The target framework identifier, the project will be built for specified tfm [default: net6.0]
--c <Debug|Release>                                         Build configuration which will be used during project build [default: Debug]
--providers <All|Gc|GcAllocHigh|GcAllocLow>                 Providers which will be used for collecting events [default: All]
--instrument <MainAssembly|MainAssemblyAndReferences|None>  Kind of instrumentation to be used [default: None]
--self-contained                                            Whether to build application in a self-contained mode [default: False]
--temp <temp>                                               Folder which will be used for temp artifacts of events collection []
--remove-temp                                               Whether to remove temp directory for artifacts after finishing work [default: True]
--arguments <arguments>                                     Arguments which will be passed when launching the program []
--arguments-file <arguments-file>                           File containing list of arguments which will be passed to program []
--print-process-output                                      Whether to print the output of a profiled application [default: True]
--filter <filter>                                           Regex to filter methods []
-?, -h, --help                                              Show help and usage information
```

## split-by-names

Usage:

```
procfiler split-by-names [options]
```

Options:

```
-p <p>                                                      The process id from which we should collect CLR events
-csproj <csproj>                                            The path to the .csproj file of the project to be executed
-o <o> (REQUIRED)                                           The output path
--duration <duration>                                       The amount of time to spend collecting CLR events [default: 60000]
--format <Csv|MethodCallTree>                               The output file(s) format [default: Csv]
--timeout <timeout>                                         The timeout which we want to wait until processing all events [default: 10000]
--clear-before                                              Clear (delete) output folder (file) before profiling session [default: True]
--merge-undefined-events                                    Should we merge events from undefined thread to managed thread events [default: True]
--tfm <tfm>                                                 The target framework identifier, the project will be built for specified tfm [default: net6.0]
--c <Debug|Release>                                         Build configuration which will be used during project build [default: Debug]
--providers <All|Gc|GcAllocHigh|GcAllocLow>                 Providers which will be used for collecting events [default: All]
--instrument <MainAssembly|MainAssemblyAndReferences|None>  Kind of instrumentation to be used [default: None]
--self-contained                                            Whether to build application in a self-contained mode [default: False]
--temp <temp>                                               Folder which will be used for temp artifacts of events collection []
--remove-temp                                               Whether to remove temp directory for artifacts after finishing work [default: True]
--arguments <arguments>                                     Arguments which will be passed when launching the program []
--arguments-file <arguments-file>                           File containing list of arguments which will be passed to program []
--print-process-output                                      Whether to print the output of a profiled application [default: True]
--filter <filter>                                           Regex to filter methods []
-?, -h, --help                                              Show help and usage information
```

## split-by-threads

Usage:

```
procfiler split-by-threads [options]
```

Options:

```
-p <p>                                                      The process id from which we should collect CLR events
-csproj <csproj>                                            The path to the .csproj file of the project to be executed
-o <o> (REQUIRED)                                           The output path
--duration <duration>                                       The amount of time to spend collecting CLR events [default: 60000]
--format <Csv|MethodCallTree>                               The output file(s) format [default: Csv]
--timeout <timeout>                                         The timeout which we want to wait until processing all events [default: 10000]
--clear-before                                              Clear (delete) output folder (file) before profiling session [default: True]
--merge-undefined-events                                    Should we merge events from undefined thread to managed thread events [default: True]
--tfm <tfm>                                                 The target framework identifier, the project will be built for specified tfm [default: net6.0]
--c <Debug|Release>                                         Build configuration which will be used during project build [default: Debug]
--providers <All|Gc|GcAllocHigh|GcAllocLow>                 Providers which will be used for collecting events [default: All]
--instrument <MainAssembly|MainAssemblyAndReferences|None>  Kind of instrumentation to be used [default: None]
--self-contained                                            Whether to build application in a self-contained mode [default: False]
--temp <temp>                                               Folder which will be used for temp artifacts of events collection []
--remove-temp                                               Whether to remove temp directory for artifacts after finishing work [default: True]
--arguments <arguments>                                     Arguments which will be passed when launching the program []
--arguments-file <arguments-file>                           File containing list of arguments which will be passed to program []
--print-process-output                                      Whether to print the output of a profiled application [default: True]
--filter <filter>                                           Regex to filter methods []
-?, -h, --help                                              Show help and usage information
```

## split-by-methods

Usage:

```
procfiler split-by-methods [options]
```

Options:

```
--repeat <repeat>                                                                        The number of times launching of the program should be repeated [default: 1]
--inline <EventsAndMethodsEvents|EventsAndMethodsEventsWithFilter|NotInline|OnlyEvents>  Should we inline inner methods calls to all previous traces [default: NotInline]
--group-async-methods                                                                    Group events from async methods [default: True]
-p <p>                                                                                   The process id from which we should collect CLR events
-csproj <csproj>                                                                         The path to the .csproj file of the project to be executed
-o <o> (REQUIRED)                                                                        The output path
--duration <duration>                                                                    The amount of time to spend collecting CLR events [default: 60000]
--format <Csv|MethodCallTree>                                                            The output file(s) format [default: Csv]
--timeout <timeout>                                                                      The timeout which we want to wait until processing all events [default: 10000]
--clear-before                                                                           Clear (delete) output folder (file) before profiling session [default: True]
--merge-undefined-events                                                                 Should we merge events from undefined thread to managed thread events [default: True]
--tfm <tfm>                                                                              The target framework identifier, the project will be built for specified tfm [default: net6.0]
--c <Debug|Release>                                                                      Build configuration which will be used during project build [default: Debug]
--providers <All|Gc|GcAllocHigh|GcAllocLow>                                              Providers which will be used for collecting events [default: All]
--instrument <MainAssembly|MainAssemblyAndReferences|None>                               Kind of instrumentation to be used [default: None]
--self-contained                                                                         Whether to build application in a self-contained mode [default: False]
--temp <temp>                                                                            Folder which will be used for temp artifacts of events collection []
--remove-temp                                                                            Whether to remove temp directory for artifacts after finishing work [default: True]
--arguments <arguments>                                                                  Arguments which will be passed when launching the program []
--arguments-file <arguments-file>                                                        File containing list of arguments which will be passed to program []
--print-process-output                                                                   Whether to print the output of a profiled application [default: True]
--filter <filter>                                                                        Regex to filter methods []
-?, -h, --help                                                                           Show help and usage information
```
