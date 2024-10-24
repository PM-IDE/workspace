FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build
ARG BUILD_CONFIGURATION=Release
ARG PROJECT_NAME=FicusFrontend

RUN apt update -y && apt upgrade -y
RUN apt install nodejs -y
RUN apt install npm -y

WORKDIR /app
COPY ./Ficus/src/front/FicusFrontend/ ./Ficus/src/front/FicusFrontend/
COPY ./Ficus/protos/ ./Ficus/protos/

RUN dotnet restore ./Ficus/src/front/FicusFrontend/$PROJECT_NAME/$PROJECT_NAME.csproj

WORKDIR /app/Ficus/src/front/FicusFrontend/$PROJECT_NAME/
RUN dotnet publish $PROJECT_NAME.csproj -c $BUILD_CONFIGURATION -o /app/publish

FROM nginx:alpine

EXPOSE 8080

ARG PROJECT_NAME=FicusFrontend

WORKDIR /var/www/web

COPY --from=build /app/publish/wwwroot .
COPY ./Ficus/src/front/FicusFrontend/$PROJECT_NAME/nginx.conf /etc/nginx/nginx.conf
