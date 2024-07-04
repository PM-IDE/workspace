# PM-IDE workspace

The repository unites different projects for modern software process mining, based on
.NET event logs. Detailed information about each project can be found in corresponding 
Readme files in each project's subdirectory.

## Procfiler

Procfiler is a tool which collects various events from .NET program execution and serializes
them either to XES format or bXES format. The event types include event types which are
supported by the TraceEvent library and method start-end events which are collected
through unmanaged profiler.

## Ficus

Ficus is a tool for process mining with much attention put to performance. Ficus consists
of Python client (`pip install ficus_pm`) and backend part written in Rust 
(`docker run -it aerooneqq/ficus:latest`). 

## bXES

bXES (binary XES, pronounced as "boxes") is a binary format for storing event logs.
The goal of creating such format is saving disk space when storing event logs, and
especially software event logs. The bXES supports converting XES event logs to bXES
(however the nested attributes are not supported).