# Helper script to launch Tauri Dev mode with automatic MSVC environment setup

$sdkVer  = "10.0.26100.0"
$msvcVer = "14.44.35207"
$msvcBase = "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\VC\Tools\MSVC\$msvcVer"
$sdkBase  = "C:\Program Files (x86)\Windows Kits\10"

$env:LIB     = "$msvcBase\lib\x64;$sdkBase\Lib\$sdkVer\um\x64;$sdkBase\Lib\$sdkVer\ucrt\x64"
$env:INCLUDE = "$msvcBase\include;$sdkBase\Include\$sdkVer\ucrt;$sdkBase\Include\$sdkVer\um;$sdkBase\Include\$sdkVer\shared"
$env:PATH    = "$msvcBase\bin\Hostx64\x64;$sdkBase\bin\$sdkVer\x64;$env:USERPROFILE\.cargo\bin;$env:PATH"

Write-Host "[DEV] Launching LE.GO.LAS Desktop in DEV mode..." -ForegroundColor Cyan
npx tauri dev
