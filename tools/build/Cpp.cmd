@echo off
setlocal
set "CTW_VCVARS=%ProgramFiles(x86)%\Microsoft Visual Studio\18\BuildTools\VC\Auxiliary\Build\vcvars64.bat"
if not exist "%CTW_VCVARS%" (
  echo Visual Studio C++ environment not found. See docs/BUILD_CPP.md.
  exit /b 1
)
call "%CTW_VCVARS%" >nul
if errorlevel 1 exit /b 1
python "%~dp0cpp.py" %*
exit /b %errorlevel%
