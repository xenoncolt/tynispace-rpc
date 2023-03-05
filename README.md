# Jellyflix-RPC

Program used to display what you are watching on discord.

This program made by [Radiicall](https://github.com/Radiicall) and If you want to make your own for your server visit [here](https://github.com/Radiicall/jellyfin-rpc).

Everything about this program will be found [here](https://github.com/Radiicall/jellyfin-rpc). If you are using [Jellyflix](https://info.jellyflix.ga) you can use this program. 

![GitHub package.json version](https://img.shields.io/github/package-json/v/xenoncolt/jellyflix-rpc?style=plastic) ![GitHub release (latest by date)](https://img.shields.io/github/v/release/xenoncolt/jellyflix-rpc?style=plastic) ![GitHub](https://img.shields.io/github/license/xenoncolt/jellyflix-rpc?style=plastic) ![GitHub all releases](https://img.shields.io/github/downloads/xenoncolt/jellyflix-rpc/total?style=plastic) 

# Example json
```json
{ 
    "Jellyfin": { 
        "USERNAME": "xenon" 
    }, 
    "Discord": { 
        "ENABLE_IMAGES": true
    } 
} 
```

# Installation Guide
- For Windows Download `jellyflix-rpc.bat` file
- After downloading open the bat file 
- Follow the instruction. 
- Enter valid information ( if you don't know check [example.json](#example-json) file )

# Remove / Uninstallation
- Before remove/ uninstall, you need to stop the `jellyflix-rpc` service. For that, Run a terminal with administration. Copy this command and hit enter :
```cmd
%APPDATA%\jellyfin-rpc\nssm-2.24\win64\nssm.exe stop jellyflix-rpc
```
- After stopping the service now you can remove the service from windows. For that, Run a terminal with administration. Copy this command and hit enter : 
```cmd
%APPDATA%\jellyfin-rpc\nssm-2.24\win64\nssm.exe remove jellyflix-rpc
```

If it is not working the creates an issue...

