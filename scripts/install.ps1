#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Installs nr (universal media converter) to the system PATH.
.DESCRIPTION
    Downloads the latest nr.exe from GitHub Releases and adds it to PATH.
    Run with: iex "& { $(irm https://raw.githubusercontent.com/subhradeepsarkae-ai/noir/main/scripts/install.ps1) }"
#>

$Repo = "subhradeepsarkae-ai/noir"
$BinDir = "$env:LOCALAPPDATA\noir\bin"
$ExePath = "$BinDir\nr.exe"

New-Item -ItemType Directory -Path $BinDir -Force | Out-Null

$LocalExe = Join-Path (Split-Path $PSScriptRoot -Parent) "target\release\nr.exe"
if (Test-Path $LocalExe) {
    Write-Host "Installing from local build..." -ForegroundColor Cyan
    Copy-Item $LocalExe $ExePath -Force
} else {
    Write-Host "Downloading nr latest release..." -ForegroundColor Cyan
    $releases = "https://api.github.com/repos/$Repo/releases/latest"
    $tag = (Invoke-RestMethod $releases).tag_name
    $url = "https://github.com/$Repo/releases/download/$tag/nr-x86_64-pc-windows-msvc.zip"
    $zip = "$env:TEMP\nr.zip"
    Invoke-WebRequest -Uri $url -OutFile $zip -UseBasicParsing
    Expand-Archive -Path $zip -DestinationPath $BinDir -Force
    Remove-Item $zip
}

$userPath = [Environment]::GetEnvironmentVariable("PATH", "User")
if ($userPath -notlike "*$BinDir*") {
    [Environment]::SetEnvironmentVariable("PATH", "$userPath;$BinDir", "User")
    Write-Host "Added $BinDir to user PATH" -ForegroundColor Green
}

$env:PATH = "$env:PATH;$BinDir"

Write-Host "OK nr installed to $ExePath" -ForegroundColor Green
Write-Host "Run 'nr --help' to get started." -ForegroundColor Cyan
