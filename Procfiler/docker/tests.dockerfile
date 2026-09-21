FROM mcr.microsoft.com/dotnet/sdk:10.0 AS build-env

COPY --from=mcr.microsoft.com/dotnet/sdk:9.0 /usr/share/dotnet /usr/share/dotnet
COPY --from=mcr.microsoft.com/dotnet/sdk:8.0 /usr/share/dotnet /usr/share/dotnet

RUN apt update -y && apt upgrade -y
RUN apt-get update -y

RUN apt-get -y install build-essential
RUN apt-get -y install ninja-build

RUN apt update
RUN apt install ca-certificates gpg wget
RUN wget -O - https://apt.kitware.com/keys/kitware-archive-latest.asc 2>/dev/null | gpg --dearmor - | tee /usr/share/keyrings/kitware-archive-keyring.gpg >/dev/null
RUN echo 'deb [signed-by=/usr/share/keyrings/kitware-archive-keyring.gpg] https://apt.kitware.com/ubuntu/ jammy main' | tee /etc/apt/sources.list.d/kitware.list >/dev/null
RUN apt update
RUN apt install cmake

WORKDIR /app
COPY ./Procfiler ./Procfiler
COPY ./bxes ./bxes
COPY ./Ficus ./Ficus
COPY ProcfilerBxes.sln ./ProcfilerBxes.sln
COPY Directory.Packages.props ./Directory.Packages.props

RUN dotnet build ./Procfiler/src/dotnet/ProcfilerBuildTasks/ProcfilerBuildTasks.csproj -c Release -v d
RUN dotnet build . -c Release -v d

FROM build-env as test
ENTRYPOINT [ "dotnet", "test", "/app/Procfiler/src/dotnet/OnlineProcfilerTests/OnlineProcfilerTests.csproj", "-c", "Release", "/p:SolutionDir=/app" ]