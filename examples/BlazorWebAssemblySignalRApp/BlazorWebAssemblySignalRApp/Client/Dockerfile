#See https://aka.ms/containerfastmode to understand how Visual Studio uses this Dockerfile to build your images for faster debugging.

FROM mcr.microsoft.com/dotnet/aspnet:5.0 AS base
WORKDIR /app
EXPOSE 80
EXPOSE 443

FROM mcr.microsoft.com/dotnet/sdk:5.0 AS build
WORKDIR /src
COPY ["BlazorWebAssemblySignalRApp/Client/BlazorWebAssemblySignalRApp.Client.csproj", "BlazorWebAssemblySignalRApp/Client/"]
COPY ["BlazorWebAssemblySignalRApp/Shared/BlazorWebAssemblySignalRApp.Shared.csproj", "BlazorWebAssemblySignalRApp/Shared/"]
RUN dotnet restore "BlazorWebAssemblySignalRApp/Client/BlazorWebAssemblySignalRApp.Client.csproj"
COPY . .
WORKDIR "/src/BlazorWebAssemblySignalRApp/Client"
RUN dotnet build "BlazorWebAssemblySignalRApp.Client.csproj" -c Release -o /app/build

FROM build AS publish
RUN dotnet publish "BlazorWebAssemblySignalRApp.Client.csproj" -c Release -o /app/publish

FROM base AS final
WORKDIR /app
COPY --from=publish /app/publish .
ENTRYPOINT ["dotnet", "BlazorWebAssemblySignalRApp.Client.dll"]
