// === imports
use polars::prelude::pivot::pivot_stable;
use polars::prelude::*;

// === main
fn main() {
    // === block_1

    // Connect to parquet
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/large/partitioned"), args).unwrap();

    // Mean income by region and age_group
    let lf = lf
        .filter(col("keep_type").eq(1))
        .filter(col("income").is_not_null())
        .group_by([col("region"), col("age_group")])
        .agg([(col("income"))
            .mean()
            .alias("mean_income")
            .round(2, RoundMode::HalfAwayFromZero)])
        .sort(["region", "age_group"], SortMultipleOptions::default());

    // Change region code to region name
    let lf = lf.with_column(col("region").replace_strict(
        lit(Series::from_iter(vec![
            "E12000001",
            "E12000002",
            "E12000003",
            "E12000004",
            "E12000005",
            "E12000006",
            "E12000007",
            "E12000008",
            "E12000009",
            "W92000004",
        ])),
        lit(Series::from_iter(vec![
            "North East",
            "North West",
            "Yorkshire and The Humber",
            "East Midlands",
            "West Midlands",
            "East of England",
            "London",
            "South East",
            "South West",
            "Wales",
        ])),
        None,
        Some(DataType::String),
    ));

    // Change age group code to age group name
    let df = lf
        .with_column(col("age_group").replace_strict(
            lit(Series::from_iter(vec![-8, 1, 2, 3, 4, 5, 6, 7])),
            lit(Series::from_iter(vec![
                "Does not apply",
                "Aged 15 years and under",
                "Aged 16 to 24 years",
                "Aged 25 to 34 years",
                "Aged 35 to 44 years",
                "Aged 45 to 54 years",
                "Aged 55 to 64 years",
                "Aged 65 years and over",
            ])),
            None,
            Some(DataType::String),
        ))
        .collect()
        .unwrap();

    println!("{}", &df);

    // === block_2

    // Pivot wider / pivot
    let df_wide = pivot_stable(
        &df,
        ["region"],
        Some(["age_group"]),
        Some(["mean_income"]),
        false,
        None,
        None,
    )
    .unwrap();

    println!("{}", &df_wide);

    // === block_3

    // Pivot longer / unpivot
    let df_long = df_wide
        .unpivot(
            [
                "North East",
                "North West",
                "Yorkshire and The Humber",
                "East Midlands",
                "West Midlands",
                "East of England",
                "London",
                "South East",
                "South West",
                "Wales",
            ],
            ["age_group"],
        )
        .unwrap();

    // Unpivot can be done lazily! Add that!

    println!("{}", &df_long);

    // === end
}
