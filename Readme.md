<br />
<div align="center">
  <a href="https://github.com/othneildrew/Best-README-Template">
    <img src="images/logo.png" alt="Logo" width="80" height="80">
  </a>
</div>


<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li>
      <a href="#getting-started">Getting Started</a>
      <ul>
        <li><a href="#prerequisites">Prerequisites</a></li>
        <li><a href="#installation">Installation</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#roadmap">Roadmap</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>



<!-- ABOUT THE PROJECT -->
## About The Project

The repository unites different projects for modern software process mining, based on
.NET event logs. Detailed information about each project can be found in corresponding
Readme files in each project's subdirectory.

### Procfiler

Procfiler is a tool which collects various events from .NET program execution and serializes
them either to XES format or bXES format. The event types include event types which are
supported by the TraceEvent library and method start-end events which are collected
through unmanaged profiler.

### Ficus

Ficus is a tool for process mining with much attention put to performance. Ficus consists
of Python client (`pip install ficus_pm`) and backend part written in Rust
(`docker run -it aerooneqq/ficus:latest`).

### bXES

bXES (binary XES, pronounced as "boxes") is a binary format for storing event logs.
The goal of creating such format is saving disk space when storing event logs, and
especially software event logs. The bXES supports converting XES event logs to bXES
(however the nested attributes are not supported).


### Built With

* Rust
* .NET
* Typescript
* Python
* C++

## Usage

See readmes in each projects sub-directories.

## Contributing

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request


<!-- LICENSE -->
## License

Distributed under the MIT License. See `LICENSE.txt` for more information.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Contact

TG: @AeroOne
Email: aerooneQ@yandex.ru, Stepanov.E.V@hse.ru

<p align="right">(<a href="#readme-top">back to top</a>)</p>


## Acknowledgments

* [PAIS Lab](https://pais.hse.ru/lab/about)
* [DotNetCoreProfiler](https://github.com/ABaboshin/DotNetCoreProfiler)
* [.NET runtime tests](https://github.com/dotnet/runtime/tree/main/src/tests/profiler)
* [.NET Profiling docs](https://github.com/dotnet/runtime/tree/main/docs/design/coreclr/profiling)
* [Perfview](https://github.com/Microsoft/perfview)

<p align="right">(<a href="#readme-top">back to top</a>)</p>