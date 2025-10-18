use df_interchange::Interchange;
// === imports
use plotlars::{Axis, BarPlot, Legend, LinePlot, Orientation, Plot, Rgb, Text};
use polars::prelude::{pivot::pivot_stable, *};

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/large/partitioned"), args).unwrap();

    // Modify var
    let lf = lf
        .filter(col("keep_type").eq(lit(1))) // Usual resident
        .filter(col("income").is_not_null());

    // Mean income by region and sex
    let df_bar = lf
        .clone()
        .group_by([col("sex"), col("region")])
        .agg([col("income").mean().round(2, RoundMode::HalfAwayFromZero)])
        .sort(["sex", "region"], SortMultipleOptions::new())
        .with_column(col("region").replace_strict(
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
        ))
        .with_column(col("sex").replace_strict(
            lit(Series::from_iter(vec!["1", "2"])),
            lit(Series::from_iter(vec!["Female", "Male"])),
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
        .labels("region")
        .values("income")
        .orientation(Orientation::Vertical)
        .group("sex")
        .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
        .plot_title(
            Text::from("Income by sex and region")
                .font("Arial")
                .size(18),
        )
        .x_title(Text::from("Region").font("Arial").size(15))
        .y_title(Text::from("Mean Income").font("Arial").size(15))
        .legend_title(Text::from("Sex").font("Arial").size(15))
        .legend(
            &Legend::new()
                .orientation(Orientation::Horizontal)
                .y(1.1)
                .x(0.37),
        )
        .build()
        .to_html();

    // === block_3

    let mut file = std::fs::File::create("./data/output/bar.html").unwrap();
    std::io::Write::write_all(&mut file, html.as_bytes()).unwrap();

    // === block_4

    // Mean income by sex and hours worked (groupped)
    let df_line = lf
        .clone()
        .filter(col("hours_worked").neq(lit(-8)))
        .group_by([col("sex"), col("hours_worked")])
        .agg([col("income").mean().round(2, RoundMode::HalfAwayFromZero)])
        .sort(["sex", "hours_worked"], SortMultipleOptions::new())
        .with_column(col("sex").replace_strict(
            lit(Series::from_iter(vec!["1", "2"])),
            lit(Series::from_iter(vec!["Female", "Male"])),
            None,
            Some(DataType::String),
        ))
        .collect()
        .unwrap();

    // Pivot to make two lines out of sex values
    let df_line = pivot_stable(
        &df_line,
        ["sex"],
        Some(["hours_worked"]),
        Some(["income"]),
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
        .x("hours_worked")
        .x_axis(
            &Axis::new()
                .tick_values(vec![1.0, 2.0, 3.0, 4.0])
                .tick_labels(vec![
                    "15 hours or less",
                    "16 to 30 hours",
                    "31 to 48 hours",
                    "49 or more hours",
                ]),
        )
        .y("Female")
        .additional_lines(vec!["Male"])
        .size(12)
        .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
        .plot_title("Mean income by hours worked (groupped) and sex")
        .y_title("Mean income")
        .x_title("Hours worked (groupped)")
        .legend_title("Sex")
        .build()
        .to_html();

    // === block_6

    let mut file = std::fs::File::create("./data/output/line.html").unwrap();
    std::io::Write::write_all(&mut file, html.as_bytes()).unwrap();

    // === end
}
