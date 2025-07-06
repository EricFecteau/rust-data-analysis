// use df_interchange::Interchange;
use polars::prelude::pivot::pivot_stable;
use polars::prelude::*;
use polars_excel_writer::PolarsExcelWriter;
use rust_xlsxwriter::{Chart, ChartLegendPosition, ChartType, Workbook};

fn main() {
    // Connect to LazyFrame (no data is brought into memory)
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet("./data/lfs_large/part", args).unwrap();

    // Modify var
    let lf: LazyFrame = lf
        .filter(col("hrlyearn").is_not_null())
        .with_column((col("hrlyearn").cast(DataType::Float64) / lit(100)).alias("hourly_wages"));

    // Mean by year and province
    let df_long = lf
        .clone()
        .group_by([col("survyear"), col("prov")])
        .agg([col("hourly_wages")
            .mean()
            .round(2, RoundMode::HalfAwayFromZero)])
        .sort(["survyear", "prov"], Default::default())
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
        .collect()
        .unwrap();

    // Pivot by province
    let df_wide = pivot_stable(
        &df_long,
        ["prov"],
        Some(["survyear"]),
        Some(["hourly_wages"]),
        false,
        None,
        None,
    )
    .unwrap();

    println!("Long:\n{df_long}");

    println!("Wide:\n{df_wide}");

    // Create a new Excel writer.
    let mut excel_writer = PolarsExcelWriter::new();

    // Create workbook
    let mut workbook = Workbook::new();

    // Write long table to "long" worksheet
    let ws_long = workbook.add_worksheet().set_name("long").unwrap();
    excel_writer
        .write_dataframe_to_worksheet(&df_long, ws_long, 0, 0)
        .unwrap();

    // Write wide table to "wide" worksheet
    let ws_wide = workbook.add_worksheet().set_name("wide").unwrap();
    excel_writer
        .write_dataframe_to_worksheet(&df_wide, ws_wide, 0, 0)
        .unwrap();

    // Add a chart sheet for the "wide" data

    // Get some info to limit the size and shape of the graph (e.g. rows, min/max values)
    let row_num = df_wide.shape().0;
    let col_num = df_wide.shape().1;
    let min_val: f64 = df_long
        .column("hourly_wages")
        .unwrap()
        .as_series()
        .unwrap()
        .min()
        .unwrap()
        .unwrap();
    let max_val: f64 = df_long
        .column("hourly_wages")
        .unwrap()
        .as_series()
        .unwrap()
        .max()
        .unwrap()
        .unwrap();

    // Iterate throw rows to create multiple lines
    let mut chart = Chart::new(ChartType::Line);
    for i in 1..col_num {
        chart
            .add_series()
            .set_name(("wide", 0, i as u16)) // Name of province found in row 1, column i
            .set_categories(("wide", 1, 0, row_num as u32, 0)) // Category (year) found in column 1, row 1-year_count
            .set_values(("wide", 1, i as u16, row_num as u32, i as u16)); // FR, FC, LR, LC
    }
    chart.x_axis().set_name("Year");
    chart
        .y_axis()
        .set_name("Hourly wages")
        .set_min((min_val - 1.0).round())
        .set_max((max_val + 1.0).round());
    chart.legend().set_position(ChartLegendPosition::Bottom);
    ws_wide.insert_chart(1, 12, &chart).unwrap();

    // Save the file to disk.
    workbook
        .save("./data/output/mean_hourly_wages.xlsx")
        .unwrap();
}
