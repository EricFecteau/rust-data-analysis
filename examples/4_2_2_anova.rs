// === imports
use hypors::anova::anova;
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/large/partitioned"), args).unwrap();

    // Income by economically active type (in London)
    let df = lf
        .clone()
        .filter(col("keep_type").eq(lit(1))) // Usual resident
        .filter(col("region").eq(lit("E12000007"))) // London
        .filter(col("income").is_null().not())
        .select([col("income"), col("econ")])
        .sort(["econ"], Default::default())
        .with_column(col("econ").replace_strict(
            lit(Series::from_iter(vec![1, 2, 3, 4])),
            lit(Series::from_iter(vec![
                "Employee",
                "Self-employed",
                "Unemployed",
                "Full-time student",
            ])),
            None,
            Some(DataType::String),
        ))
        .with_row_index("index", None)
        .collect()
        .unwrap();

    println!("{}", &df);

    // === block_2

    // Transpose
    let df = pivot::pivot_stable(
        &df,
        ["econ"],
        Some(["index"]),
        Some(["income"]),
        false,
        None,
        None,
    )
    .unwrap()
    .drop("index")
    .unwrap();

    println!("{}", &df);

    // === block_3

    // Create Vec<Vec<f64>> for ANOVA
    let cols = df
        .get_columns()
        .iter()
        .map(|c| {
            c.as_materialized_series()
                .to_float()
                .unwrap()
                .f64()
                .unwrap()
                .drop_nulls()
                .to_vec_null_aware()
                .left()
                .unwrap()
        })
        .collect::<Vec<Vec<f64>>>();

    // === block_4

    // Perform one-way ANOVA
    let alpha = 0.05;
    let result = anova(&cols, alpha).unwrap();

    println!(
        "F-statistic: {}\nP-value: {}\nNull hypothesis: {}\nReject null: {}",
        result.test_statistic, result.p_value, result.null_hypothesis, result.reject_null
    );

    // === end
}
