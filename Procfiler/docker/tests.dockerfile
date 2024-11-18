FROM mcr.microsoft.com/dotnet/sdk:9.0 AS build-env

RUN apt update -y && apt upgrade -y
RUN apt-get update -y

RUN apt-get -y install build-essential
RUN apt-get -y install ninja-build
RUN apt-get -y install cmake

WORKDIR /app
COPY ./Procfiler ./Procfiler
COPY ./bxes ./bxes
COPY ProcfilerBxes.sln ./ProcfilerBxes.sln

RUN dotnet build ./Procfiler/src/dotnet/ProcfilerBuildTasks/ProcfilerBuildTasks.csproj -c Release
RUN dotnet build . -c Release

FROM build-env as test
ENTRYPOINT [ "dotnet", "test", "/app/Procfiler/src/dotnet/OnlineProcfilerTests/OnlineProcfilerTests.csproj", "-c", "Release", "/p:SolutionDir=/app" ]