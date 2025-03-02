#!/bin/bash

start_year=2011
end_year=2024

rm -r ./data
mkdir -p ./data
mkdir -p ./data/lfs_csv
mkdir -p ./data/lfs_parquet
mkdir -p ./data/lfs_large

for y in $(seq $start_year $end_year); do
    curl https://www150.statcan.gc.ca/n1/pub/71m0001x/2021001/hist/$y-CSV.zip --output ./data/$y.zip
    unzip ./data/$y.zip -d ./data/temp
    for m in 01 02 03 04 05 06 07 08 09 10 11 12; do
        mv ./data/temp/pub$m${y: -2}.csv ./data/lfs_csv/pub$m${y: -2}.csv
    done
    rm -r ./data/temp
    rm ./data/$y.zip
done

