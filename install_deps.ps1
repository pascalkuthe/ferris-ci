# Check if choco is installed

if (!(Get-Command "choco" -errorAction SilentlyContinue)) {
    Write-Error "choco is not installed, get it at: https://chocolatey.org/install"
}

function install ([string]$cmd, [string]$package, [string]$extra_args){
    if (Get-Command $cmd -errorAction SilentlyContinue) {
        return
    }

    if ($package -eq ""){
        $package = $cmd
    }

    Write-Output "${package} (${cmd}) is not installed, installing with choco"

    if ($extra_args -eq ""){
        choco install -y --no-progress $package
    }else{
        choco install -y --no-progress $package --installargs "${extra_args}"
    }
}

install clang llvm
install ninja
install cmake cmake "ADD_CMAKE_TO_PATH=User"
install tar
install zstd zstandard
install gpg
install aws awscli

RefreshEnv
