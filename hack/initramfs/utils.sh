#!/bin/bash

copy_lib() {
    cp -Lr $1 `echo $1 | awk '{print substr($1,2); }'`
}

install_libs() {
    libs=$(ldd $1 \
        | grep so \
        | sed -e '/^[^\t]/ d' \
        | sed -e 's/\t//' \
        | sed -e 's/.*=..//' \
        | sed -e 's/ (0.*)//' \
        | sort \
        | uniq -c \
        | sort -n \
        | awk '{$1=$1;print}' \
        | cut -d' ' -f2 \
        | grep "^/")
    
    for l in ${libs[@]}; do
        copy_lib $l
    done
}
