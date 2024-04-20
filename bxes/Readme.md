## This document describes the `bxes` format.

### Goals

The goal of creating the format is to provide a compact way of storing event logs in Process Mining field, especially in
software field.
The main problem about software logs is their size, as each event can contain a lot of attributes and the number of
events is big. Moreover,
some attributes values can be repeated many times (e.g. name of a method), thus, when using XES format, there will a lot
of repetition, which leads
to enormous size of .xes files. `bxes` format aims to provide a binary representation of event logs, thus reducing the
size and optimizing the process
of working with software event logs.

### Event log description

Event log is a sequence of traces, or, the `multiset` of traces. This indicates, that some traces can be repeated for
several times.
An event log can contain meta-information, e.g. version of format, date of creation, etc.
Every trace is a sequence of events.
Trace may not contain any metadata as event log does.
Every event contains a set of attributes, where each attribute contains a name of attribute (`String`) and a value of
the attribute (any of primitive types).
The core difference between `bxes` and other XML-like formats is that we do not allow complex data structures and nested
tags.
E.g. the constructs like the following one are not allowed:

```xml
<event>
    <usermetadata>
        <string key = "concept-name" value = "John" />
        <int key = "age" value = "12" />
    </usermetadata>
</event>
```

In `bxes` every event contains a plain set of attributes, in other words every attribute is a
pair `(name: String, value: PrimitiveType)`, and an event is
a set of such pairs.
Event may not contain metadata, as event log does, all information about an event should be stored in the set of its
attributes.

### Core features of `bxes`

The bxes core features are:

- Aggressive reuse of attribute keys and values: instead of repeating them as in XES, the actual value will be stored
  once, while attributes in an event
  will reference those values.
- Aggressive reuse of attribute pairs
- Attribute values and each trace variant can be stored in a single file, or in separate files.
- Reuse of traces variants: i.e. two traces for a single trace variant will not be stored

### Type system

The following types are supported in bxes:

- `NULL` (type id = 0, `0 bytes`)
- `i32` (type id = 1, `4 bytes`)
- `i64` (type id = 2, `8 bytes`)
- `u32` (type id = 3, `4 bytes`)
- `u64` (type id = 4, `8 bytes`)
- `f32` (type id = 5, `4 bytes`)
- `f64` (type id = 6, `8 bytes`)
- `String` (UTF-8 strings) (type id = 7, length bytes) + (length in bytes, `u64`)
- `bool` (type id = 8, `1 byte`)

XES-sprcific types:

- `timestamp` (type id = 9, `8 bytes`), the date is i64 which represents the number of nanoseconds sine Unix epoch
- `braf-lifecycle-transition` (type id = 10, `1 byte`) - BRAF lifecycle model
    - NULL (unspecified) = `0`,
    - Closed = `1`
    - Closed.Cancelled = `2`
    - Closed.Cancelled.Aborted = `3`
    - Closed.Cancelled.Error = `4`
    - Closed.Cancelled.Exited = `5`
    - Closed.Cancelled.Obsolete = `6`
    - Closed.Cancelled.Terminated = `7`
    - Completed = `8`
    - Completed.Failed = `9`
    - Completed.Success = `10`
    - Open = `11`
    - Open.NotRunning = `12`
    - Open.NotRunning.Assigned = `13`
    - Open.NotRunning.Reserved = `14`
    - Open.NotRunning.Suspended.Assigned = `15`
    - Open.NotRunning.Suspended.Reserved = `16`
    - Open.Running = `17`
    - Open.Running.InProgress = `18`
    - Open.Running.Suspended = `19`
- `standard-lifecycle-transition` (type id = 11, `1 byte`) - standard lifecycle model
    - NULL (unspecified) = `0`,
    - assign = `1`
    - ate_abort = `2`
    - autoskip = `3`
    - complete = `4`
    - manualskip = `5`
    - pi_abort = `6`
    - reassign = `7`
    - resume = `8`
    - schedule = `9`
    - start = `10`
    - suspend = `11`
    - unknown = `12`
    - withdraw = `13`
- `artifact` (type id = `12`) xes artifact extension
    - the number of models is written (`u32`, `4 bytes`)
    - then the models are written
        - each model is a value-value-value triple, which indicates the values
          of `artifactlifecycle:model`, `artifactlifecycle:instance` and `artifactlifecycle:transition`
- `cost:dirvers` (type id = `13`) xes cost extension. The list of drivers with following attributes:
    - the number of drivers is written (`u32`, `4 bytes`), each list item is the following:
        - the amount is written (`f64`, `8 bytes`)
        - the driver name index is written (`u32`, `4 bytes`)
        - the type index is written (`u32`, `4 bytes`)
- `guid` (type id = `14`) the guid written in LE order, `16 bytes`
- `software event type` (type id = `15`, `1 byte`) - enum:
    - NULL = `0`,
    - Call = `1`
    - Return = `2`
    - Throws = `3`
    - Handle = `4`
    - Calling = `5`
    - Returning = `6`

Type id is one byte length. In case of string the length of a string in bytes is also serialized, the length of string
takes 8 bytes.
Type id + additional type info (i.e. length of a string) forms a header of a value, followed by the actual value

### Single file format description

- The version of bxes is specified (`u32`) - `4 bytes`
- System metadata is written
- The number of values is written (`u32`) - `4 bytes`
- Then there is a sequence of values [(Header[type-id + metainfo], value)]
- Then there is a number of attribute key-values pairs (`u32`) - `4 bytes`
- After that there is a sequence of pairs (index(`u32`, `4 bytes`), index(`u32`, `4 bytes`)), which indicates the
  attributes key-value pairs.
- The event log metadata is written
- Then the number of traces variants is written (`u32`) - `4 bytes`
- Then the sequence of traces variants is written.

### System metadata format
- The number of value-attributes is written (`u32`) - `4 bytes`
  - The value-attributes info are written:
    - The name of the attribute is written (`String`)
    - The type of the attribute is written (`byte`)

### Event log metadata format

- The number of properties is written (`u32`)
- The properties are written: key-value pairs, key must be of type string
- The number of extensions is written (`u32`)
- The extensions are written. The extension is tuple of three elements (name index (`u32`), prefix index (`u32`), uri
  index (`u32`))
- The number of entities for globals are written (`u8`)
- The globals are written:
    - The entity identifier (`u8`)
    - The number globals is written (`u32`)
    - The globals are written: key-value pairs
    - Type of entities:
        - event = `0`
        - trace = `1`
        - log = `2`
- The number of classifiers is written (`u32`)
- The classifiers are written
    - The name index of a classifier is written
    - The number of keys is written (`u32`)
    - The keys are written (value-index `u32`)

### Trace variant format

- The number of traces is written (`u32`)
- The number of trace metadata is written (`u32`)
- The metadata is written: key-value pairs
- The number of events is written (`u32`)
- The events are written

### Event description

- name value index: (`u32`, `4 bytes`)
- timestamp value (`i64`, `8 bytes`), number of nanoseconds since Unix Epoch (date in UTC)
- the value-attributes are written, the number of value attributes is specified in the event log metadata.
- number of attributes (`u32`, `4 bytes`)
- sequence of key value indices (size * `4 bytes`)

### Multiple files format description

- System metadata file
    - The version of bxes is written (`u32`, `4 bytes`)
    - The system metadata is written
- Metadata file
    - The version of bxes is written (`u32`, `4 bytes`)
    - The metadata is written
- Values file
    - The version of bxes is written (`u32`, `4 bytes`)
    - The number of values is written (`u32`, `4 bytes`)
    - The values are written [(Header[type-id + metainfo], value)]
- Key-value pairs file
    - The version of bxes is written (`u32`, `4 bytes`)
    - The number of key-value pairs is written (`u32`, `4 bytes`)
    - The key-value pairs are written (index(`u32`, `4 bytes`), index(`u32`, `4 bytes`))
- Traces file
    - The version of bxes is written (`u32`, `4 bytes`)
    - The trace variant is written

### Online event log transfer

The opportunity to divide event log into different files can help in online transferring of event logs.
The core idea is that values, attribute key-value pairs, event log metadata and traces can be transferred independently.