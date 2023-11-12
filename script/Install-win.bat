@echo off

REM Set paths
set EXE_PATH=%APPDATA%\tynispace-rpc\tynispace-rpc.exe
set JSON_PATH=%APPDATA%\tynispace-rpc\main.json
set DOWNLOAD_URL=https://github.com/xenoncolt/tynispace-rpc/releases/latest/download/tynispace-rpc.exe
set DOWNLOAD_DIR=%APPDATA%\tynispace-rpc

REM set
set JELLYFIN_USERNAME=YOUR_TYNISPACE_USERNAME
set DISCORD_ENABLE_IMAGES=true

echo ===============================================================================
echo                        TYNISPACE-RPC INSTALLATION 
echo ===============================================================================
echo.

REM Check if jellyflix-rpc folder exist
if not exist "%DOWNLOAD_DIR%" mkdir "%DOWNLOAD_DIR%"

REM Check if jellyflix-rpc.exe is present
if exist "%EXE_PATH%" (
    echo tynispace-rpc.exe is already present. & timeout /t 3 /nobreak >nul
) else (
    REM Downloading jellyflix-rpc binary
    echo Downloading tynispace-rpc binary from GitHub... & timeout /t 3 /nobreak >nul
    curl -L %DOWNLOAD_URL% -o "%DOWNLOAD_DIR%\tynispace-rpc.exe"
)

rem Prompt the user for input
set /p JELLYFIN_USERNAME=Enter TyniSpace username "[%JELLYFIN_USERNAME%]": 
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
%DOWNLOAD_DIR%\nssm-2.24\win64\nssm.exe install tynispace-rpc "%EXE_PATH%" "-c %JSON_PATH%"

REM Start the executable using NSSM
echo Starting tynispace-rpc service... & timeout /t 3 /nobreak >nul
powershell -Command "Start-Process cmd -Verb RunAs -ArgumentList '/c net start tynispace-rpc'"

REM Pause for 5 seconds
ping -n 5 127.0.0.1 > nul

REM Coded by xenoncolt.xyz

REM Check if the service is running
tasklist /fi "imagename eq tynispace-rpc.exe" | find ":" > nul
if %errorlevel%==0 (
    echo ===============================================================================
    echo                      TYNISPACE-RPC SERVICE IS RUNNING
    echo ===============================================================================
) else (
    echo tynispace-rpc service failed to start.
)

echo.
echo ===============================================================================
echo                            INSTALLATION COMPLETE!
echo ===============================================================================
pause
exit