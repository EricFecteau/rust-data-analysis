// :dep polars = { version = "0.49", features = ["lazy", "parquet", "regex", "replace"] }

use polars::prelude::*;

fn main() {
    // Connect and process multiple monthly parquet file
    let mut lfs_month = vec![];
    for m in 1..5 {
        let mm = format!("{m:02}");

        let args = ScanArgsParquet::default();
        let lf =
            LazyFrame::scan_parquet(format!("./data/lfs_parquet/pub{mm}23.parquet"), args).unwrap();

        lfs_month.push(lf);
    }

    // Concatenate vertically two (or more) datasets
    let lf_jan_to_apr = concat(
        [
            lfs_month[0].clone(), // Cloned, since we need it later
            lfs_month[1].clone(),
            lfs_month[2].clone(),
            lfs_month[3].clone(),
        ],
        UnionArgs::default(),
    )
    .unwrap();

    // See `survmnth` going from 1 to 4 for 2023
    println!("{}", lf_jan_to_apr.collect().unwrap());

    // Update the lfs_month vector to remove variables and update values
    for m in 1..5 {
        let mm = format!("{m:02}");

        lfs_month[m - 1] = lfs_month[m - 1]
            .clone()
            .filter(col("hrlyearn").is_not_null())
            .select([
                col("rec_num"),
                col("survyear"),
                col("hrlyearn").alias(format!("earn_{mm}")),
            ]);
    }

    // Left join (creating a cohort)
    let jan_cohort = lfs_month[0]
        .clone()
        .drop([col("survyear")])
        .left_join(
            lfs_month[1].clone().drop([col("survyear")]),
            col("rec_num"),
            col("rec_num"),
        )
        .left_join(
            lfs_month[2].clone().drop([col("survyear")]),
            col("rec_num"),
            col("rec_num"),
        )
        .left_join(
            lfs_month[3].clone().drop([col("survyear")]),
            col("rec_num"),
            col("rec_num"),
        );

    println!("{}", jan_cohort.collect().unwrap());

    // Inner join (creating a "always earning" cohort)
    let longitudinal_all = lfs_month[0]
        .clone()
        .drop([col("survyear")])
        .inner_join(
            lfs_month[1].clone().drop([col("survyear")]),
            col("rec_num"),
            col("rec_num"),
        )
        .inner_join(
            lfs_month[2].clone().drop([col("survyear")]),
            col("rec_num"),
            col("rec_num"),
        )
        .inner_join(
            lfs_month[3].clone().drop([col("survyear")]),
            col("rec_num"),
            col("rec_num"),
        );

    println!("{}", longitudinal_all.collect().unwrap());

    // More complex types of joins (e.g. join on multiple variables)
    let fix_full_join_vars = [
        when(col("rec_num").is_not_null())
            .then(col("rec_num"))
            .otherwise(col("rec_num_right"))
            .alias("rec_num"),
        when(col("survyear").is_not_null())
            .then(col("survyear"))
            .otherwise(col("survyear_right"))
            .alias("survyear"),
    ];

    let longitudinal_all = lfs_month[0]
        .clone()
        .join(
            lfs_month[1].clone(),
            [col("rec_num"), col("survyear")],
            [col("rec_num"), col("survyear")],
            JoinArgs::new(JoinType::Full),
        )
        .with_columns(fix_full_join_vars.clone())
        .drop([col("rec_num_right"), col("survyear_right")])
        .join(
            lfs_month[2].clone(),
            [col("rec_num"), col("survyear")],
            [col("rec_num"), col("survyear")],
            JoinArgs::new(JoinType::Full),
        )
        .with_columns(fix_full_join_vars.clone())
        .drop([col("rec_num_right"), col("survyear_right")])
        .join(
            lfs_month[3].clone(),
            [col("rec_num"), col("survyear")],
            [col("rec_num"), col("survyear")],
            JoinArgs::new(JoinType::Full),
        )
        .with_columns(fix_full_join_vars.clone())
        .drop([col("rec_num_right"), col("survyear_right")])
        .sort(["rec_num", "survyear"], Default::default());

    println!("{}", longitudinal_all.collect().unwrap());
}
