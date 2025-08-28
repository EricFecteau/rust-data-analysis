// === evcxr

// === imports
use std::env;
use std::fs::File;
use std::io::prelude::*;

use comrak::{Options, markdown_to_html};
use plotlars::{BarPlot, Legend, Orientation, Plot, Rgb, Text};
use polars::prelude::{pivot::pivot_stable, *};

// === main
fn main() {
    // === block_1

    let mut file = File::create("./data/output/report.html").unwrap();

    // === block_2

    let mut html = r#"<!DOCTYPE html>
                            <html>
                            <head>
                            <style>
                                body {background-color: #36454F; ;}
                                div {width: 900px;}
                                h1 {color: white;}
                                h2 {color: white;}
                                p {
                                    color: white;
                                    text-align: justify;
                                }
                                table, th, td {
                                    color: white;
                                    border: 1px solid;
                                    border-collapse: collapse;
                                    padding: 3px;
                                    text-align: left;
                                }
                                table {
                                    margin-left: none;
                                    margin-right: auto;
                                }
                                img {
                                    display: block;
                                    margin-left: none;
                                    width: 80%;
                                }
                            </style>
                            </head>
                            <body>
                            <div>"#
        .to_string();

    // === block_3

    unsafe {
        env::set_var("POLARS_FMT_TABLE_FORMATTING", "MARKDOWN");
        env::set_var("POLARS_FMT_TABLE_HIDE_COLUMN_DATA_TYPES", "1");
        env::set_var("POLARS_FMT_TABLE_HIDE_DATAFRAME_SHAPE_INFORMATION", "1");
        env::set_var("POLARS_FMT_MAX_COLS", "11"); // Provinces + Gender
    }

    // === block_4

    let mut markdown = "# Title of Report \n".to_string();

    markdown.push_str("## Intro \n");

    markdown.push_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus laoreet ornare dolor, eget iaculis diam tempor sit amet. Ut non diam euismod, dapibus ligula at, vulputate eros. Integer mattis leo at ligula interdum dignissim. Nam at justo id ipsum venenatis lacinia ac id diam. Aenean sit amet urna suscipit tortor varius iaculis non quis ligula. Vestibulum sed nunc eu elit dictum rutrum. Vivamus sit amet mattis dolor, a ultricies urna. Nam viverra, purus at dictum accumsan, est justo mattis turpis, ultrices faucibus nibh libero ac risus. Praesent et lectus imperdiet lectus consectetur convallis. Sed vitae velit tortor. Aliquam nibh magna, auctor et lectus porta, laoreet cursus nisi.

Maecenas posuere tellus porttitor turpis pulvinar luctus. Nunc lacinia suscipit nisl nec efficitur. Praesent in purus vitae quam viverra interdum. Pellentesque ut blandit lorem, et mattis odio. Maecenas suscipit cursus rhoncus. Maecenas consectetur mauris libero, malesuada efficitur tellus condimentum in. Suspendisse ornare odio in est malesuada egestas. Fusce vel magna nibh. Nam molestie ut sem porta viverra. Vivamus ut convallis orci. Suspendisse ornare hendrerit sodales. Cras ipsum massa, eleifend in ullamcorper at, ullamcorper eget ligula. \n");

    // === block_5

    markdown.push_str("## Data \n");

    markdown.push_str("Donec mollis faucibus finibus. Etiam hendrerit odio accumsan, egestas purus et, tristique sem. Praesent metus velit, molestie fringilla nulla hendrerit, lobortis egestas velit. Duis vitae metus quis diam egestas convallis sed ac nunc. Duis dignissim risus a diam aliquam, sit amet pulvinar justo volutpat. Sed eget cursus arcu. Etiam eget ornare ipsum. Aliquam egestas dignissim odio vel sagittis.\n\n");

    // Connect to LazyFrame
    let args = ScanArgsParquet::default();
    let lf = LazyFrame::scan_parquet(PlPath::from_str("./data/lfs_large/part"), args).unwrap();

    // Modify var
    let lf = lf
        .filter(col("hrlyearn").is_not_null())
        .with_column((col("hrlyearn").cast(DataType::Float64) / lit(100)).alias("hourly_wages"));

    // Mean hourly wage by province and gender
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

    // Pivot to make two lines out of gender values
    let df_bar_trans = pivot_stable(
        &df_bar,
        ["prov"],
        Some(["gender"]),
        Some(["hourly_wages"]),
        false,
        None,
        None,
    )
    .unwrap();

    markdown.push_str("**Table 1**: Hourly wages by gender and province \n");
    markdown.push_str(&df_bar_trans.to_string());
    markdown.push_str("\n\n");

    // === block_6

    markdown.push_str("## Graphic \n");

    markdown.push_str("Sed pharetra quis tellus ut porta. Aliquam maximus neque aliquet elit rhoncus feugiat. Fusce scelerisque elit in quam accumsan feugiat. Vivamus et venenatis neque, non congue ligula. In sit amet mollis eros. Quisque justo velit, luctus elementum nisi vel, lobortis eleifend mauris. Curabitur eget posuere augue, et facilisis ipsum.\n\n");

    BarPlot::builder()
        .data(&df_bar)
        .labels("prov")
        .values("hourly_wages")
        .orientation(Orientation::Vertical)
        .group("gender")
        .colors(vec![Rgb(255, 127, 80), Rgb(64, 224, 208)])
        .plot_title(
            Text::from("Chart 1: Hourly wages by gender and province")
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
        .write_image("./data/output/out.png", 1000, 600, 1.0)
        .unwrap();

    markdown.push_str("![alt text](out.png) \n");

    // === block_7

    markdown.push_str("## Conclusion \n");

    markdown.push_str(" In elit libero, hendrerit sit amet placerat et, posuere id justo. Cras eleifend porta ex eu varius. Orci varius natoque penatibus et magnis dis parturient montes, nascetur ridiculus mus. Nunc id nisi eleifend, sollicitudin est a, maximus metus. Nulla urna nisi, suscipit vitae tincidunt sit amet, aliquam a dui. Donec pellentesque, nisi a ornare convallis, nibh odio posuere quam, ac consequat mauris nisi sit amet lacus. Proin sit amet condimentum enim. Maecenas fringilla enim a est dignissim, sed vehicula tellus euismod. Duis vitae lobortis enim.\n\n");

    // === block_8

    let mut options = Options::default();
    options.extension.table = true;

    html.push_str(markdown_to_html(markdown.as_str(), &options).as_str());
    html.push_str(
        r#"</div>
                  </body>
                  </html>"#,
    );

    file.write_all(html.as_bytes()).unwrap();

    // === end
}
