Description:

Usage:
procfiler [command] [options]

Options:
--version Show version information
-?, -h, --help Show help and usage information

Commands:

- create-documentation
    - Generate documentation for filters, mutators and other application components
- collect
    - Collects the CLR events from the given process ID
- collect-to-xes
    - Collect CLR events and serialize them to XES format
- meta-info
    - Collects meta-information about CLR events received during listening to process
- undefined-events-to-xes
    - Collect CLR events from undefined thread and serialize them to XES format
- split-by-names
    - Split the events into different files based on managed thread ID
- split-by-threads
    - Split the events into different files based on managed thread ID
- split-by-methods
    - Splits the events by methods, in which they occured, and serializes to XES
