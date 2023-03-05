@echo off

REM Set paths
set EXE_PATH=%APPDATA%\jellyflix-rpc\jellyflix-rpc.exe
set JSON_PATH=%APPDATA%\jellyflix-rpc\main.json
set DOWNLOAD_URL=https://github.com/xenoncolt/jellyflix-rpc/releases/latest/download/jellyflix-rpc.exe
set DOWNLOAD_DIR=%APPDATA%\jellyflix-rpc

REM set
set JELLYFIN_USERNAME=YOUR_JELLYFLIX_USERNAME
set DISCORD_ENABLE_IMAGES=true

echo ===============================================================================
echo                        JELLYFLIX-RPC INSTALLATION 
echo ===============================================================================
echo.

REM Check if jellyflix-rpc folder exist
if not exist "%DOWNLOAD_DIR%" mkdir "%DOWNLOAD_DIR%"

REM Check if jellyflix-rpc.exe is present
if exist "%EXE_PATH%" (
    echo jellyflix-rpc.exe is already present. & timeout /t 3 /nobreak >nul
) else (
    REM Downloading jellyflix-rpc binary
    echo Downloading jellyflix-rpc binary from GitHub... & timeout /t 3 /nobreak >nul
    curl -L %DOWNLOAD_URL% -o "%DOWNLOAD_DIR%\jellyflix-rpc.exe"
)

rem Prompt the user for input
set /p JELLYFIN_USERNAME=Enter Jellyflix username "[%JELLYFIN_USERNAME%]": 
set /p DISCORD_ENABLE_IMAGES=Enable Discord images (true/false) [%DISCORD_ENABLE_IMAGES%]:


rem Output the JSON data to the file
echo { > main.json
echo     "Jellyfin": { >> main.json
echo         "USERNAME": "%JELLYFIN_USERNAME%" >> main.json
echo     }, >> main.json
echo     "Discord": { >> main.json
echo         "ENABLE_IMAGES": %DISCORD_ENABLE_IMAGES% >> main.json
echo     } >> main.json
echo } >> main.json

REM Check if main.json is present
if exist "%JSON_PATH%" (
    echo main.json file is already present & timeout /t 3 /nobreak >nul
    del "main.json"
) else (
    move "main.json" "%DOWNLOAD_DIR%\"
)

REM Check if NSSM is already installed
if exist "%DOWNLOAD_DIR%\nssm-2.24\win64\nssm.exe" (
    echo NSSM is already installed. & timeout /t 3 /nobreak >nul
) else (
    REM Download NSSM installer
    echo Downloading and unzipping NSSM installer... & timeout /t 3 /nobreak >nul
    curl -L https://nssm.cc/release/nssm-2.24.zip -o nssm.zip

    REM Unzip NSSM
    powershell -Command "Expand-Archive -LiteralPath nssm.zip -DestinationPath ."
    move /Y "nssm-2.24" "%DOWNLOAD_DIR%\"
    echo Deleting unnecessary nssm.zip file
    del "nssm.zip"
)

REM Install NSSM
echo Installing jellyflix-rpc service... & timeout /t 3 /nobreak >nul
%DOWNLOAD_DIR%\nssm-2.24\win64\nssm.exe install jellyflix-rpc "%EXE_PATH%"

REM Start the executable using NSSM
echo Starting jellyflix-rpc service... & timeout /t 3 /nobreak >nul
set "psCommand=powershell -Command "Start-Process %DOWNLOAD_DIR%\nssm-2.24\win64\nssm.exe -Verb RunAs -ArgumentList 'start','jellyflix-rpc'""
powershell -NoProfile -ExecutionPolicy Bypass -Command "%psCommand%"

REM Pause for 5 seconds
ping -n 5 127.0.0.1 > nul

REM Coded by xenoncolt.tk

REM Check if the service is running
tasklist /fi "imagename eq jellyflix-rpc.exe" | find ":" > nul
if %errorlevel%==0 (
    echo ===============================================================================
    echo                      JELLYFLIX-RPC SERVICE IS RUNNING
    echo ===============================================================================
) else (
    echo jellyflix-rpc service failed to start.
)

echo.
echo ===============================================================================
echo                            INSTALLATION COMPLETE!
echo ===============================================================================
