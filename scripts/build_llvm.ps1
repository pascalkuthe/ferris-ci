# init development environment
Push-Location .
& "C:\Program Files (x86)\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\Launch-VsDevShell.ps1" -Arch amd64 -HostArch amd64
Pop-Location

cargo ferris-ci build llvm 16.0.6 -j 8 --upload
