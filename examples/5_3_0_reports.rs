// === evcxr

// === imports
use std::env;
use std::fs::File;
use std::io::prelude::*;

use polars::prelude::*;

use comrak::{Options, markdown_to_html};

// === main
fn main() {
    // === block_1

    // ScatterPlot::builder()
    //     .data(&df_scatter)
    //     .x("age_12")
    //     .y("hourly_wages")
    //     .group("gender")
    //     .opacity(0.5)
    //     .size(12)
    //     .colors(vec![Rgb(178, 34, 34), Rgb(65, 105, 225), Rgb(255, 140, 0)])
    //     .plot_title("Penguin Flipper Length vs Body Mass")
    //     .x_title("Body Mass (g)")
    //     .y_title("Flipper Length (mm)")
    //     .legend_title("Species")
    //     .build()
    //     .write_image("./data/output/out.png", 800, 600, 1.0)
    //     .unwrap();

    let mut file = File::create("./data/output/report.html").unwrap();

    let mut html = r#"<!DOCTYPE html>
                            <html>
                            <head>
                            <style>
                                body {background-color: #36454F; ;}
                                div {width: 900px;}
                                h1 {color: white;}
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
                                    margin-left: auto;
                                    margin-right: auto;
                                }
                                img {
                                    display: block;
                                    margin: auto;
                                    width: 80%;
                                }
                            </style>
                            </head>
                            <body>
                            <div>"#
        .to_string();

    let mut markdown = "# Hello World\nThis is a new line\n".to_string();

    unsafe {
        env::set_var("POLARS_FMT_TABLE_FORMATTING", "MARKDOWN");
        env::set_var("POLARS_FMT_TABLE_HIDE_COLUMN_DATA_TYPES", "1");
        env::set_var("POLARS_FMT_TABLE_HIDE_DATAFRAME_SHAPE_INFORMATION", "1");
    }

    markdown.push_str("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Vivamus laoreet ornare dolor, eget iaculis diam tempor sit amet. Ut non diam euismod, dapibus ligula at, vulputate eros. Integer mattis leo at ligula interdum dignissim. Nam at justo id ipsum venenatis lacinia ac id diam. Aenean sit amet urna suscipit tortor varius iaculis non quis ligula. Vestibulum sed nunc eu elit dictum rutrum. Vivamus sit amet mattis dolor, a ultricies urna. Nam viverra, purus at dictum accumsan, est justo mattis turpis, ultrices faucibus nibh libero ac risus. Praesent et lectus imperdiet lectus consectetur convallis. Sed vitae velit tortor. Aliquam nibh magna, auctor et lectus porta, laoreet cursus nisi.

Maecenas posuere tellus porttitor turpis pulvinar luctus. Nunc lacinia suscipit nisl nec efficitur. Praesent in purus vitae quam viverra interdum. Pellentesque ut blandit lorem, et mattis odio. Maecenas suscipit cursus rhoncus. Maecenas consectetur mauris libero, malesuada efficitur tellus condimentum in. Suspendisse ornare odio in est malesuada egestas. Fusce vel magna nibh. Nam molestie ut sem porta viverra. Vivamus ut convallis orci. Suspendisse ornare hendrerit sodales. Cras ipsum massa, eleifend in ullamcorper at, ullamcorper eget ligula. ");

    // Creating columns
    let year = Column::new("year".into(), [2010, 2010, 2011, 2011, 2011, 2011]);
    let month = Column::new(
        "month".into(),
        [
            "November", "December", "January", "February", "March", "April",
        ],
    );
    let value = Column::new("value".into(), [1.25, 2.50, 3.75, 4.00, 3.75, 4.25]);

    let df = DataFrame::new(vec![year, month, value]).unwrap();

    let str = df.to_string();

    println!("{str}");

    markdown.push_str("\n\n\n");
    markdown.push_str(&str);
    markdown.push_str("\n\n\n");
    markdown.push_str("![alt text](/home/eric/Rust/rust-data-analysis/data/output/out.png)");

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
