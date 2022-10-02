# Visual Studio installation directory
$VS_DIR = & "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe" -property installationPath

# init development environment
Push-Location .
& "$VS_DIR\Common7\Tools\Launch-VsDevShell.ps1" -Arch amd64 -HostArch amd64
Pop-Location

cargo ferris-ci build llvm 14.0.6 -j 8 --upload
