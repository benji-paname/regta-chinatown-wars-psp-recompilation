param([switch]$Test, [switch]$Check, [switch]$FullGame)
$ErrorActionPreference = 'Stop'
$cppArguments = @('--runtime')
if ($Check) { $cppArguments = @('--check') }
if ($Test) { $cppArguments += '--test' }
if ($FullGame) { $cppArguments += '--full-game' }
& (Join-Path $PSScriptRoot 'tools/build/Cpp.cmd') @cppArguments
if ($LASTEXITCODE -ne 0) { throw "C++ build failed ($LASTEXITCODE). Playtest and saves have not been replaced." }
