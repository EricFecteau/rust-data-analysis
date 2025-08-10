// use std::{env, fmt::Debug};

// use polars::prelude::*;

// use markdown2pdf::parse_into_file;

fn main() {
    let mut _markdown = "# Hello World\nThis is a test.".to_string();

    // unsafe {
    //     env::set_var("POLARS_FMT_TABLE_FORMATTING", "MARKDOWN");
    // }

    // // Creating columns
    // let year = Column::new("year".into(), [2010, 2010, 2011, 2011, 2011, 2011]);
    // let month = Column::new(
    //     "month".into(),
    //     [
    //         "November", "December", "January", "February", "March", "April",
    //     ],
    // );
    // let value = Column::new("value".into(), [1.25, 2.50, 3.75, 4.00, 3.75, 4.25]);

    // let df = DataFrame::new(vec![year, month, value]).unwrap();

    // println!("{df}");

    // let str = df.to_string();

    // println!("{str}");

    // markdown.push_str("\n\n\n");
    // markdown.push_str(&str);

    // parse_into_file(markdown, "./data/output/output.pdf", None).unwrap();
}
