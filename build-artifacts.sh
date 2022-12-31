#!/bin/bash

for target in $(rustup target list | awk -v FS='(' '/installed/{ print $1 }'); do
  ARCH=$(echo $target | awk -v FS='-' '{ print $1 }');
  BASENAME=quizzer

  if [[ -n $(echo $target | grep "windows") ]]; then
    ARTIFACT_NAME=$BASENAME.exe
    FILENAME=$BASENAME-$ARCH.exe
  else
    ARTIFACT_NAME=$BASENAME
    FILENAME=$BASENAME.$ARCH
  fi

  cargo build --release --target $target

  cp ./target/$target/release/$ARTIFACT_NAME ./bin/$FILENAME
done