#!/usr/bin/env bash

if [ $2 ]; then
  sourceDir=$2
else
  sourceDir=$PWD/src/
fi

if [ $1 ]; then
  echo -e "\033[32m> 即将编译: ${sourceDir/$PWD/.}$1.rs \033[0m"
else
  echo -e "\033[31m> Error: 未提供源文件名 \033[0m"
  exit 1
fi

baseDir=$PWD/src/
targetDir=./target/${sourceDir/$baseDir/""}

if [ ! -d $targetDir ]; then
  mkdir -p $targetDir
elif [ -a "$targetDir$1" ]; then
  rm $targetDir$1
fi

# 核心编译语句
rustc $sourceDir$1.rs -o $targetDir$1

if [ -a $targetDir$1 ]; then
  echo -e "\033[32m> 编译成功: $targetDir$1 \n \033[0m"

  $targetDir$1
else
  echo -e "\033[31m> Error: 编译失败！ \033[0m"
fi
