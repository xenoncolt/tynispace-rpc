# Tynispace-RPC

Program used to display what you are watching on discord.

This program made by [Radiicall](https://github.com/Radiicall) and If you want to make your own for your server visit [here](https://github.com/Radiicall/jellyfin-rpc).

Everything about this program will be found [here](https://github.com/Radiicall/jellyfin-rpc). If you are using [Tynispace](https://info.jellyflix.ga) you can use this program. 

![GitHub package.json version](https://img.shields.io/github/package-json/v/xenoncolt/tynispace-rpc?style=plastic) ![GitHub release (latest by date)](https://img.shields.io/github/v/release/xenoncolt/tynispace-rpc?style=plastic) ![GitHub](https://img.shields.io/github/license/xenoncolt/tynispace-rpc?style=plastic) ![GitHub all releases](https://img.shields.io/github/downloads/xenoncolt/tynispace-rpc/total?style=plastic) 

# Example json
```json
{ 
    "Jellyfin": { 
        "USERNAME": "YOUR_JELLYFLIX_USERNAME" 
    }, 
    "Discord": { 
        "ENABLE_IMAGES": true
    } 
} 
```

# Installation Guide
- For Windows Download `tynispace-rpc.bat` file
- After downloading open the bat file 
- Follow the instruction. 
- Enter valid information ( if you don't know check [example.json](#example-json) file )

# Remove / Uninstallation
- Before remove/ uninstall, you need to stop the `tynispace-rpc` service. For that, Run a terminal with administration. Copy this command and hit enter :
```cmd
%APPDATA%\tynispace-rpc\nssm-2.24\win64\nssm.exe stop tynispace-rpc
```
- After stopping the service now you can remove the service from windows. For that, Run a terminal with administration. Copy this command and hit enter : 
```cmd
%APPDATA%\tynispace-rpc\nssm-2.24\win64\nssm.exe remove tynispace-rpc
```

### License
<details>
<summary>Click to expand</summary>

[GPL-3.0](https://github.com/xenoncolt/tynispace-rpc/blob/main/LICENSE)
</details>

### Contact
<details>
<summary>Click to expand</summary>

If you have any questions, comments, or concerns, please create a Issue [here](https://github.com/xenoncolt/tynispace-rpc/issues)
</details>
