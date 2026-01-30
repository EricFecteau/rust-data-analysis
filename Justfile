compile-all:
    cargo build -r --examples

process-book: 
    rm -rf ./src_processed
    mkdir src_processed
    cp -r ./src/* ./src_processed
    cd book_processor && cargo run -r

render: process-book
    mdbook serve --open

build: process-book
    mdbook build

test-all: get-data test-rw test-trans test-stats test-pub

kill-minio:
    pkill minio

get-data:
    rm -rf ./data
    cargo run -r --example 1_2_1_extract
    cargo run -r --example 1_2_2_rename
    cargo run -r --example 1_2_3_synthetic
    cargo run -r --example 1_2_4_expand
    cargo run -r --example 1_2_5_parquet
    cargo run -r --example 1_2_6_large
    cargo run -r --example 1_2_7_sql
    minio server ./data/minio --quiet &
    cargo run -r --example 1_2_8_minio
    pkill minio

test-rw:
    cargo run -r --example 2_1_1_dataframe
    cargo run -r --example 2_2_1_read_csv
    cargo run -r --example 2_2_2_write_csv
    cargo run -r --example 2_3_1_read_parquet
    cargo run -r --example 2_3_2_write_parquet
    cargo run -r --example 2_3_3_write_partitioned_parquet
    cargo run -r --example 2_4_1_postgresql
    cargo run -r --example 2_4_2_sql_to_polars
    minio server ./data/minio --quiet &
    cargo run -r --example 2_5_1_read_cloud
    cargo run -r --example 2_5_2_write_cloud
    pkill minio

test-trans:
    cargo run -r --example 3_1_1_filter
    cargo run -r --example 3_1_2_filter_opt
    cargo run -r --example 3_2_1_select
    cargo run -r --example 3_3_1_variables
    cargo run -r --example 3_4_1_pivot
    cargo run -r --example 3_5_1_joins

test-stats:
    cargo run -r --example 4_1_1_summary
    cargo run -r --example 4_2_1_chi_square
    cargo run -r --example 4_2_2_anova
    cargo run -r --example 4_2_3_mwu

test-pub:
    cargo run -r --example 5_1_1_excel
    cargo run -r --example 5_2_1_plots
    cargo run -r --example 5_3_1_reports