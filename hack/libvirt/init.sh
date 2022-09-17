#!/bin/bash
set -e

thisDir=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
targetDir=$(realpath "$thisDir/../../")


. ${thisDir}/../../hack/kernel/config.sh
cat ${thisDir}/libvirt.xml | sed "s#__PWD__#${targetDir}#" | sed "s#__KERNEL_VERSION__#${KERNEL_VERSION}#" | tee ${thisDir}/../../target/libvirt.xml
