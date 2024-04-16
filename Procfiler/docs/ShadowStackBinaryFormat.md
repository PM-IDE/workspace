This document describes the binary format of storing shadow-stacks.

## Assumptions

- ~~Method ID doesn't equal to 0~~
- Method details does not present in this file. Methods details should be retrievable from method information which
  is logged through EventPipe
- Method ID which is logged in shadow stack equals to method ID in EventPipe
- ~~Managed Thread ID doesn't equal to 0~~

## Format

File contains shadow-stacks for all managed threads. Shadow-stack for a managed thread ID is represented by
a sequence of open and close frames, managed thread id and number of frames. 
Open frame indicates that the method has began. Close frame indicates that a method has finished.

Frame is a combination of indication of start or end of the method. Then the method ID is written. Thus frame
is two numbers:
- Indication of start or end of method invocation. `1` for start, `0` for end. `1 byte` length
- Method ID - `long` (`8 bytes`) on every platform.

Managed thread is a `long` number before a shadow-stack.

Thus the file has a following structure:

```
[ManagedThread1ID (8 bytes)][NumberOfFrames (8 bytes)][Frames...(n bytes)]
[ManagedThread2ID (8 bytes)][NumberOfFrames (8 bytes)][Frames... (m bytes)]

...

[ManagedThreadNID (8 bytes)][NumberOfFrames (8 bytes)][Frames... (k bytes)]
 ```
