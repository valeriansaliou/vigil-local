#!/bin/bash

##
#  Vigil Local
#
#  Vigil local probe relay
#  Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
#  License: Mozilla Public License v2.0 (MPL v2.0)
##

# Read arguments
while [ "$1" != "" ]; do
    argument_key=`echo $1 | awk -F= '{print $1}'`
    argument_value=`echo $1 | awk -F= '{print $2}'`

    case $argument_key in
        -v | --version)
            VIGIL_LOCAL_VERSION="$argument_value"
            ;;
        *)
            echo "Unknown argument received: '$argument_key'"
            exit 1
            ;;
    esac

    shift
done

# Ensure release version is provided
if [ -z "$VIGIL_LOCAL_VERSION" ]; then
  echo "No Vigil Local release version was provided, please provide it using '--version'"

  exit 1
fi

# Define release pipeline
function release_for_architecture {
    final_tar="v$VIGIL_LOCAL_VERSION-$1.tar.gz"
    gpg_signer="valerian@valeriansaliou.name"

    rm -rf ./vigil-local/ && \
        RUSTFLAGS="-C link-arg=-s" cross build --target "$2" --release && \
        mkdir ./vigil-local && \
        cp -p "target/$2/release/vigil-local" ./vigil-local/ && \
        cp ./config.cfg vigil-local/ && \
        tar -czvf "$final_tar" ./vigil-local && \
        rm -r ./vigil-local/ && \
        gpg -u "$gpg_signer" --armor --detach-sign "$final_tar"
    release_result=$?

    if [ $release_result -eq 0 ]; then
        echo "Result: Packed architecture: $1 to file: $final_tar"
    fi

    return $release_result
}

# Run release tasks
ABSPATH=$(cd "$(dirname "$0")"; pwd)
BASE_DIR="$ABSPATH/../"

rc=0

pushd "$BASE_DIR" > /dev/null
    echo "Executing release steps for Vigil Local v$VIGIL_LOCAL_VERSION..."

    release_for_architecture "x86_64" "x86_64-unknown-linux-musl" && \
        release_for_architecture "armv7" "armv7-unknown-linux-musleabihf"
    rc=$?

    if [ $rc -eq 0 ]; then
        echo "Success: Done executing release steps for Vigil Local v$VIGIL_LOCAL_VERSION"
    else
        echo "Error: Failed executing release steps for Vigil Local v$VIGIL_LOCAL_VERSION"
    fi
popd > /dev/null

exit $rc
