param([switch]$Test, [string]$ToolchainRoot = $env:CTW_RUST_TOOLCHAIN, [switch]$Offline)
$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path (Split-Path $PSScriptRoot -Parent) -Parent
& (Join-Path $repoRoot 'tools/build/Compile.ps1') @PSBoundParameters