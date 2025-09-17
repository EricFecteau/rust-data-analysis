// === evcxr

use df_interchange::Interchange;
// === imports
use plotlars::{BarPlot, Legend, LinePlot, Orientation, Plot, Rgb, Text};
use polars::prelude::{pivot::pivot_stable, *};

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/part"), args).unwrap();

    // Modify var
    let lf = lf
        .filter(col("hrlyearn").is_not_null())
        .with_column((col("hrlyearn").cast(DataType::Float64) / lit(100)).alias("hourly_wages"));

    // Mean by province and gender
    let df_bar = lf
        .clone()
        .group_by([col("gender"), col("prov")])
        .agg([col("hourly_wages")
            .mean()
            .round(2, RoundMode::HalfAwayFromZero)])
        .sort(["gender", "prov"], SortMultipleOptions::new())
        .with_column(col("prov").replace_strict(
            lit(Series::from_iter(vec![
                "10", "11", "12", "13", "24", "35", "46", "47", "48", "59",
            ])),
            lit(Series::from_iter(vec![
                "NL", "PE", "NS", "NB", "QC", "ON", "MB", "SK", "AB", "BC",
            ])),
            None,
            Some(DataType::String),
        ))
        .with_column(col("gender").replace_strict(
            lit(Series::from_iter(vec!["1", "2"])),
            lit(Series::from_iter(vec!["Men+", "Women+"])),
            None,
            Some(DataType::String),
        ))
        .collect()
        .unwrap();

    println!("{df_bar}");

    // Convert from Polars 0.51 to Polars 0.50
    let df_bar = Interchange::from_polars_0_51(df_bar)
        .unwrap()
        .to_polars_0_50()
        .unwrap();

    // === block_2

    let html = BarPlot::builder()
        .data(&df_bar)
        .labels("prov")
        .values("hourly_wages")
        .orientation(Orientation::Vertical)
        .group("gender")
        .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
        .plot_title(
            Text::from("Hourly wages by gender and province")
                .font("Arial")
                .size(18),
        )
        .x_title(Text::from("Province").font("Arial").size(15))
        .y_title(Text::from("Mean hourly wage").font("Arial").size(15))
        .legend_title(Text::from("Gender").font("Arial").size(15))
        .legend(
            &Legend::new()
                .orientation(Orientation::Horizontal)
                .y(1.0)
                .x(0.4),
        )
        .build()
        .to_html();

    // === block_3

    let mut file = std::fs::File::create("./data/output/bar.html").unwrap();
    std::io::Write::write_all(&mut file, html.as_bytes()).unwrap();

    // === block_4

    // Mean hourly wage by gender and tenure
    let df_line = lf
        .clone()
        .group_by([col("gender"), col("tenure")])
        .agg([col("hourly_wages")
            .mean()
            .round(2, RoundMode::HalfAwayFromZero)])
        .sort(["gender", "tenure"], SortMultipleOptions::new())
        .with_column(col("gender").replace_strict(
            lit(Series::from_iter(vec!["1", "2"])),
            lit(Series::from_iter(vec!["Men+", "Women+"])),
            None,
            Some(DataType::String),
        ))
        .collect()
        .unwrap();

    // Pivot to make two lines out of gender values
    let df_line = pivot_stable(
        &df_line,
        ["gender"],
        Some(["tenure"]),
        Some(["hourly_wages"]),
        false,
        None,
        None,
    )
    .unwrap();

    println!("{df_line}");

    // Convert from Polars 0.51 to Polars 0.50
    let df_line = Interchange::from_polars_0_51(df_line)
        .unwrap()
        .to_polars_0_50()
        .unwrap();

    // === block_5

    let html = LinePlot::builder()
        .data(&df_line)
        .x("tenure")
        .y("Men+")
        .additional_lines(vec!["Women+"])
        .size(12)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .plot_title("Mean hourly wage by job tenure and gender")
        .x_title("Mean hourly wage")
        .y_title("Job tenure (months)")
        .legend_title("Gender")
        .build()
        .to_html();

    // === block_6

    let mut file = std::fs::File::create("./data/output/line.html").unwrap();
    std::io::Write::write_all(&mut file, html.as_bytes()).unwrap();

    // === end
}
