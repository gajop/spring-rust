#!/bin/bash

LOG_LEVEL="DEBUG"

rm -rf build
mkdir build
#cmake . -G Ninja -DCMAKE_INSTALL_PREFIX=~/opt/spring -DCMAKE_BUILD_TYPE=Debug -Bbuild -DLOG_LEVEL=$LOG_LEVEL
# cmake . -G Ninja -DCMAKE_INSTALL_PREFIX=~/opt/spring-maintenance -Bbuild # -DLOG_LEVEL=$LOG_LEVEL #-DCMAKE_BUILD_TYPE=Debug
# cmake . -DCMAKE_INSTALL_PREFIX=~/opt/spring-maintenance -Bbuild -DCMAKE_BUILD_TYPE=Debug -DCMAKE_EXPORT_COMPILE_COMMANDS=1 # -DLOG_LEVEL=$LOG_LEVEL

# DEBUG_MAX_WARNINGS is too spamy as it includes sign conversion warnings which Spring has too many of
# DEBUG_SIGNAL_NANS makes Spring crash each time due to OpenAL error (fixed in new version)

cmake . -G Ninja -DCMAKE_INSTALL_PREFIX=~/opt/spring -Bbuild -DCMAKE_BUILD_TYPE=Debug -DCMAKE_EXPORT_COMPILE_COMMANDS=1 -DDEBUG_MAX_WARNINGS=0 -DDEBUG_SIGNAL_NANS=0 # -DLOG_LEVEL=$LOG_LEVEL
# pushd build
# make -j12 install spring
# popd
