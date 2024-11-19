FROM mcr.microsoft.com/dotnet/sdk:9.0 AS build
ARG BUILD_CONFIGURATION=Release
ARG PROJECT_NAME=FrontendBackend

WORKDIR /app
COPY ./Ficus/src/front/FicusFrontend/ ./Ficus/src/front/FicusFrontend/
COPY ./Ficus/protos/ ./Ficus/protos/

RUN dotnet restore ./Ficus/src/front/FicusFrontend/$PROJECT_NAME/$PROJECT_NAME.csproj

WORKDIR /app/Ficus/src/front/FicusFrontend/$PROJECT_NAME
RUN dotnet build $PROJECT_NAME.csproj -c $BUILD_CONFIGURATION -o /app/build

FROM mcr.microsoft.com/dotnet/aspnet:9.0
ARG PROJECT_NAME=FrontendBackend

EXPOSE 8080
EXPOSE 8081

WORKDIR /app
COPY --from=build /app/build .
ENV PROJECT_NAME_ENV=$PROJECT_NAME

ENTRYPOINT dotnet $PROJECT_NAME_ENV.dll