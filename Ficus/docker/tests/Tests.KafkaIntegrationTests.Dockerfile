FROM mcr.microsoft.com/dotnet/sdk:9.0 AS build-env

RUN apt update -y && apt upgrade -y
RUN apt install nodejs -y
RUN apt install npm -y

WORKDIR /app
COPY ./Ficus ./Ficus
COPY ./bxes ./bxes
COPY ./FicusBxes.sln ./FicusBxes.sln

RUN dotnet build . -c Release

FROM build-env as test
ENTRYPOINT [ "dotnet", "test", "/app/Ficus/test/IntegrationTests/IntegrationTests/IntegrationTests.csproj", "-c", "Release" ]