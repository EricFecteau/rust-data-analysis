// === evcxr

// === imports
use plotlars::{BarPlot, Legend, Orientation, Plot, Rgb, ScatterPlot, Text};
use polars::prelude::*;

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
        .x_title(Text::from("").font("Arial").size(15))
        .y_title(Text::from("hourly_wages").font("Arial").size(15))
        .legend_title(Text::from("gender").font("Arial").size(15))
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

    // Mean by gender and age (grouped)
    let df_scatter = lf
        .clone()
        .group_by([col("age_12"), col("gender")])
        .agg([col("hourly_wages")
            .mean()
            .round(2, RoundMode::HalfAwayFromZero)])
        .with_column(col("gender").replace_strict(
            lit(Series::from_iter(vec!["1", "2"])),
            lit(Series::from_iter(vec!["Men+", "Women+"])),
            None,
            Some(DataType::String),
        ))
        .collect()
        .unwrap();

    println!("{df_scatter}");

    // === block_5

    let html = ScatterPlot::builder()
        .data(&df_scatter)
        .x("age_12")
        .y("hourly_wages")
        .group("gender")
        .opacity(0.5)
        .size(12)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .plot_title("Penguin Flipper Length vs Body Mass")
        .x_title("Body Mass (g)")
        .y_title("Flipper Length (mm)")
        .legend_title("Species")
        .build()
        .to_html();

    // === block_6

    let mut file = std::fs::File::create("./data/output/scatter.html").unwrap();
    std::io::Write::write_all(&mut file, html.as_bytes()).unwrap();

    ScatterPlot::builder()
        .data(&df_scatter)
        .x("age_12")
        .y("hourly_wages")
        .group("gender")
        .opacity(0.5)
        .size(12)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .plot_title("Penguin Flipper Length vs Body Mass")
        .x_title("Body Mass (g)")
        .y_title("Flipper Length (mm)")
        .legend_title("Species")
        .build()
        .write_image("./data/output/out.png", 800, 600, 1.0)
        .unwrap();

    // === end
}
