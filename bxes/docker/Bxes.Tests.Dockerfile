FROM mcr.microsoft.com/dotnet/sdk:9.0 AS build

COPY ./bxes /bxes
COPY Directory.Packages.props ./Directory.Packages.props

WORKDIR /bxes/src/csharp/Bxes.Tests
ENTRYPOINT dotnet test