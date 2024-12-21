use polars::prelude::*;

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Filtering the data in one step
    let lf_filt = lf.clone().filter(
        col("survyear")
            .eq(lit(2010))
            .and(col("survmnth").gt(lit(6)))
            .and(col("hrlyearn").is_not_null()),
    );

    println!("{}", lf_filt.explain(false).unwrap());
    println!("{}", lf_filt.explain(true).unwrap());
    println!("{}", lf_filt.limit(5).collect().unwrap());

    //Filtering the data in multiple steps
    let lf_filt = lf
        .clone()
        .filter(col("survyear").eq(lit(2010)))
        .filter(col("survmnth").gt(lit(6)))
        .filter(col("hrlyearn").is_not_null());

    println!("{}", lf_filt.explain(false).unwrap());
    println!("{}", lf_filt.explain(true).unwrap());
    println!("{}", lf_filt.limit(5).collect().unwrap());
}
