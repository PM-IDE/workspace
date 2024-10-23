FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build-env

RUN apt-get update \
  && apt-get -y install build-essential \
  && apt-get install -y wget \
  && rm -rf /var/lib/apt/lists/* \
  && wget https://github.com/Kitware/CMake/releases/download/v3.24.1/cmake-3.24.1-Linux-x86_64.sh \
      -q -O /tmp/cmake-install.sh \
      && chmod u+x /tmp/cmake-install.sh \
      && mkdir /opt/cmake-3.24.1 \
      && /tmp/cmake-install.sh --skip-license --prefix=/opt/cmake-3.24.1 \
      && rm /tmp/cmake-install.sh \
      && ln -s /opt/cmake-3.24.1/bin/* /usr/local/bin

RUN apt-get update && apt-get -y install ninja-build

WORKDIR /app
COPY ./Procfiler ./Procfiler
COPY ./bxes ./bxes
COPY ProcfilerBxes.sln ./ProcfilerBxes.sln

RUN dotnet build . -c Release
RUN dotnet build ./Procfiler/src/dotnet/ProcfilerBuildTasks/ProcfilerBuildTasks.csproj -c Release

FROM build-env as test
ENTRYPOINT [ "dotnet", "test", "/app/Procfiler/src/dotnet/OnlineProcfilerTests/OnlineProcfilerTests.csproj", "-c", "Release", "/p:SolutionDir=/app" ]