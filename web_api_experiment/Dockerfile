FROM mcr.microsoft.com/dotnet/aspnet:6.0-focal AS base
WORKDIR /app

ENV ASPNETCORE_URLS=http://+:5131

FROM mcr.microsoft.com/dotnet/sdk:6.0-focal AS build
WORKDIR /src
COPY . .
FROM build AS publish
RUN dotnet publish "web_api_experiment.csproj" -c Release -o /app/publish /p:UseAppHost=false

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "web_api_experiment.dll"]
