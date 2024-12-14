#!/bin/bash

start_year=2006
current_year=2024
current_month=09 # Latest month the LFS is available

rm -r ./data
mkdir -p ./data
mkdir -p ./data/lfs_csv
mkdir -p ./data/lfs_parquet
mkdir -p ./data/lfs_large

for m in 01 02 03 04 05 06 07 08 09 10 11 12; do
    curl https://www150.statcan.gc.ca/n1/en/pub/71m0001x/2021001/$current_year-$m-CSV.zip --output ./data/$current_year-$m.zip
    unzip ./data/$current_year-$m.zip -d ./data/temp
    mv ./data/temp/pub$m${current_year: -2}.csv ./data/lfs_csv/pub$m${current_year: -2}.csv
    rm -r ./data/temp
    rm ./data/$current_year-$m.zip

    if [[ $m == $current_month ]]
    then
        break
    fi
done

for y in $(seq $start_year $((current_year-1))); do
    curl https://www150.statcan.gc.ca/n1/pub/71m0001x/2021001/hist/$y-CSV.zip --output ./data/$y.zip
    unzip ./data/$y.zip -d ./data/temp
    for m in 01 02 03 04 05 06 07 08 09 10 11 12; do
        mv ./data/temp/pub$m${y: -2}.csv ./data/lfs_csv/pub$m${y: -2}.csv
    done
    rm -r ./data/temp
    rm ./data/$y.zip
done

