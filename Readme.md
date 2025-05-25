[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![MIT License][license-shield]][license-url]

<br />
<div align="center">
  <a href="https://github.com/PM-IDE/workspace">
    <img src="images/FCS.svg" alt="Logo" width="80" height="80">
  </a>
</div>

<details>
  <summary>Table of Contents</summary>
  <ol>
    <li>
      <a href="#about-the-project">About The Project</a>
      <ul>
        <li><a href="#procfiler">Procfiler</a></li>
        <li><a href="#ficus">Ficus</a></li>
        <li><a href="#bxes">bXES</a></li>
        <li><a href="#built-with">Built With</a></li>
      </ul>
    </li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
    <li><a href="#contact">Contact</a></li>
    <li><a href="#acknowledgments">Acknowledgments</a></li>
  </ol>
</details>

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

* [![Rust][rust-shield]][rust-url]
* [![.NET][net-shield]][net-url]
* [![Typescript][typescript-shield]][typescript-url]
* [![Python][python-shield]][python-url]
* [![C++][cpp-shield]][cpp-url]

[rust-shield]: https://shields.io/badge/-Rust-3776AB?style=flat&logo=rust
[rust-url]: https://www.rust-lang.org/
[net-shield]: https://img.shields.io/badge/.NET-5C2D91?style=badge&logo=.net&logoColor=white
[net-url]: https://dotnet.microsoft.com/ru-ru/
[typescript-shield]: https://shields.io/badge/TypeScript-3178C6?logo=TypeScript&logoColor=FFF&style=flat-square
[typescript-url]: https://www.typescriptlang.org/
[python-shield]: https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54
[python-url]: https://www.python.org/
[cpp-shield]: https://img.shields.io/badge/-C++-blue?logo=cplusplus
[cpp-url]: https://en.cppreference.com/w/cpp/language

## Usage

See readmes in each projects sub-directories.

## Contributing

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

### Top contributors:

<a href="https://github.com/PM-IDE/workspace/graphs/contributors">
  <img src="https://contrib.rocks/image?repo=PM-IDE/workspace" alt="contrib.rocks image" />
</a>

<p align="right">(<a href="#readme-top">back to top</a>)</p>



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

[contributors-shield]: https://img.shields.io/github/contributors/PM-IDE/workspace.svg?style=for-the-badge
[contributors-url]: https://github.com/PM-IDE/workspace/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/PM-IDE/workspace.svg?style=for-the-badge
[forks-url]: https://github.com/PM-IDE/workspace/network/members
[stars-shield]: https://img.shields.io/github/stars/PM-IDE/workspace.svg?style=for-the-badge
[stars-url]: https://github.com/PM-IDE/workspace/stargazers
[issues-shield]: https://img.shields.io/github/issues/PM-IDE/workspace.svg?style=for-the-badge
[issues-url]: https://github.com/PM-IDE/workspace/issues
[license-shield]: https://img.shields.io/github/license/PM-IDE/workspace.svg?style=for-the-badge
[license-url]: https://github.com/PM-IDE/workspace/blob/master/LICENSE.txt