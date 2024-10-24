#!/bin/bash

build_variant() {
    local variant=$1
    local output_dir="target/debian"
    
    echo "Building ${variant} package..."
    if [ "$variant" = "default" ]; then
        cargo deb
    else
        cargo deb --variant=$variant
    fi
    
}

default_path=$(build_variant "default")
lite_path=$(build_variant "lite")
datapusher_plus_path=$(build_variant "datapusher_plus")

echo "DEB_PATHS=${default_path} ${lite_path} ${datapusher_plus_path}"