
# Data

This section helps you get the data that is used in the examples in this book. 

This book uses Statistics Canada's Labour Force Survey (LFS) Public User Microdata File (PUMF) as data source. These CSVs contains non-aggregated data for a wide variety of variables collected from the LFS. The LFS collects monthly information on the labour market activities of Canada's working age population.

There are multiple advantages to using this file:
* Licensed under [Statistics Canada Open License](https://www.statcan.gc.ca/en/reference/licence);
* Contains real world data, collected for a survey;
* Contains weights to reproduce the Canadian population;
* Each month of data contains a relatively small number of records (~100,000 records), but multiple years of data can be concatenated to create a fairly sizable dataset (all the way back to January 2006);
* Each month contains over 50 variables.

You can download the CSVs from [Statistics Canada's website](https://www150.statcan.gc.ca/n1/en/catalogue/71M0001X).

Here is a short bash script to download all data necessary for this book. It creates approximately 2.6 GB of CSV data. Run the bash script in the top-level folder of a cargo package.

```bash
#!/bin/bash

current_year=2024
current_month=09 # Latest month the LFS is available

rm -r ./data
mkdir -p ./data
mkdir -p ./data/statcan

for m in 01 02 03 04 05 06 07 08 09 10 11 12; do
    curl https://www150.statcan.gc.ca/n1/en/pub/71m0001x/2021001/$current_year-$m-CSV.zip --output ./data/$current_year-$m.zip
    unzip ./data/$current_year-$m.zip -d ./data/temp
    mv ./data/temp/pub$m${current_year: -2}.csv ./data/statcan/pub$m${current_year: -2}.csv
    rm -r ./data/temp
    rm ./data/$current_year-$m.zip

    if [[ $m == $current_month ]]
    then
        break
    fi
done

for y in $(seq 2006 $((current_year-1))); do
    curl https://www150.statcan.gc.ca/n1/pub/71m0001x/2021001/hist/$y-CSV.zip --output ./data/$y.zip
    unzip ./data/$y.zip -d ./data/temp
    for m in 01 02 03 04 05 06 07 08 09 10 11 12; do
        mv ./data/temp/pub$m${y: -2}.csv ./data/statcan/pub$m${y: -2}.csv
    done
    rm -r ./data/temp
    rm ./data/$y.zip
done
```

**Source**: Statistics Canada, *Labour Force Survey: Public Use Microdata File*, January 2006 to present. Reproduced and distributed on an "as is" basis with the permission of Statistics Canada.