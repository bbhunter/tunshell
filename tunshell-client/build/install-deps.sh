#!/bin/bash

set -e

TEMPDIR=${TEMPDIR:="$(dirname $0)/tmp"}
cd $TEMPDIR

SUDO="sudo"

if [[ ! -x "$(command -v sudo)" ]]; then
 SUDO=""
fi

echo "Installing compile toolchain..."
case "$OSTYPE" in
  msys*)    
    choco install rust-ms
    echo '##[add-path]%USERPROFILE%\.cargo\bin'
    ;;
  
  darwin*)    
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
    ;;
    
  FreeBSD*)
    $SUDO pkg update
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
    source $HOME/.cargo/env
    ;;
    
  *)
    $SUDO apt update -y
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
    ;;
esac

echo "Installing cross..."
cargo install cross

