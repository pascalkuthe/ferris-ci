# Visual Studio installation directory
$VS_DIR = & "C:\Program Files (x86)\Microsoft Visual Studio\Installer\vswhere.exe" -property installationPath

# init development environment
Push-Location .
& "$VS_DIR\Common7\Tools\Launch-VsDevShell.ps1" -Arch amd64 -HostArch amd64
Pop-Location

# function build-llvm-msvc {
$current_dir = (Get-Location)
$source_dir = "$current_dir\llvm"
$build_dir = "$current_dir\build"
$install_dir = "$current_dir\out"
$enable_assertions = $args[0]

# Construct a build directory and run cmake
New-Item -ItemType "directory" -Force -Path $build_dir
cmake -GNinja `
    -S "$source_dir\llvm" `
    -B $build_dir `
    -Bbuild -DCMAKE_C_COMPILER:PATH="clang-cl.exe" `
    -DCMAKE_CXX_COMPILER:PATH="clang-cl.exe" `
    -DCMAKE_RC_COMPILER:PATH="llvm-rc.exe" `
    -DCMAKE_LINKER:PATH="lld-link.exe" `
    -DLLVM_PARALLEL_LINK_JOBS=1 `
    -DLLVM_TARGETS_TO_BUILD="AArch64;ARM;X86" `
    -DLLVM_OPTIMIZED_TABLEGEN=ON `
    -DLLVM_ENABLE_LLD=ON `
    -DLLVM_ENABLE_PROJECTS=clang `
    -DCMAKE_BUILD_TYPE=Release `
    -DCMAKE_INSTALL_PREFIX="$install_dir" `
    -DLLVM_ENABLE_LIBXML2=OFF `
    -DLLVM_ENABLE_ZLIB=OFF `
    -DLLVM_ENABLE_ASSERTIONS=$enable_assertions `
    #-DLLVM_USE_CRT_RELEASE=MT

# Build the project
New-Item -ItemType "directory" -Force -Path $install_dir
if ($enable_assertions -eq "ON"){
    $config = "RelWithDebInfo"
}else{
    $config = "Release"
}
cmake --build $build_dir --target install --config $config
# }
#glpat-AEXWFys3PTqEKsyJ311Z
