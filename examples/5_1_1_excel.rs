// === imports
use polars::prelude::pivot::pivot_stable;
use polars::prelude::*;
use polars_excel_writer::PolarsExcelWriter;
use rust_xlsxwriter::{Chart, ChartLegendPosition, ChartType, Workbook};

// === main
fn main() {
    // === block_1

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/large/partitioned"), args).unwrap();

    // Filter
    let lf: LazyFrame = lf
        .filter(col("keep_type").eq(lit(1))) // Usual resident
        .filter(col("income").is_not_null());

    // Mean income by region and economic activity type
    let df_long = lf
        .clone()
        .group_by([col("region"), col("econ")])
        .agg([col("income").mean().round(2, RoundMode::HalfAwayFromZero)])
        .sort(["region", "econ"], Default::default())
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
        .collect()
        .unwrap();

    // Pivot by economic activity type
    let df_wide = pivot_stable(
        &df_long,
        ["econ"],
        Some(["region"]),
        Some(["income"]),
        false,
        None,
        None,
    )
    .unwrap();

    println!("Long:\n{df_long}");

    println!("Wide:\n{df_wide}");

    // === block_2

    // Create a new Excel writer.
    let mut excel_writer = PolarsExcelWriter::new();

    // Create workbook
    let mut workbook = Workbook::new();

    // Write long table to "long" worksheet
    let ws_long = workbook.add_worksheet().set_name("long").unwrap();
    excel_writer
        .write_dataframe_to_worksheet(&df_long, ws_long, 0, 0)
        .unwrap();

    // === block_3

    // Write wide table to "wide" worksheet
    let ws_wide = workbook.add_worksheet().set_name("wide").unwrap();
    excel_writer
        .write_dataframe_to_worksheet(&df_wide, ws_wide, 0, 0)
        .unwrap();

    // === block_4

    // Add a chart sheet for the "wide" data

    // Get some info to limit the size and shape of the graph (e.g. rows, min/max values)
    let row_num = df_wide.shape().0;
    let col_num = df_wide.shape().1;
    let min_val: f64 = df_long
        .column("income")
        .unwrap()
        .as_series()
        .unwrap()
        .min()
        .unwrap()
        .unwrap();
    let max_val: f64 = df_long
        .column("income")
        .unwrap()
        .as_series()
        .unwrap()
        .max()
        .unwrap()
        .unwrap();

    // Iterate throw rows to create multiple Bars
    let mut chart = Chart::new(ChartType::Bar);
    for i in 1..col_num {
        chart
            .add_series()
            .set_name(("wide", 0, i as u16)) // Name of region found in row 1, column i
            .set_categories(("wide", 1, 0, row_num as u32, 0)) // Category (econ) found in column 1, row 1-year_count
            .set_values(("wide", 1, i as u16, row_num as u32, i as u16)); // FR, FC, LR, LC
    }
    chart.x_axis().set_name("Economic Activity Type");
    chart
        .y_axis()
        .set_name("Income")
        .set_min((min_val - 1.0).round())
        .set_max((max_val + 1.0).round());
    chart.legend().set_position(ChartLegendPosition::Bottom);
    ws_wide.insert_chart(1, 6, &chart).unwrap();

    // === block_5

    // Save the file to disk.
    workbook.save("./data/output/income.xlsx").unwrap();

    // // === end
}
