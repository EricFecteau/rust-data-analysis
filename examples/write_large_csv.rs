use human_bytes::human_bytes;
use polars::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let current_year = 2024;
    let current_month = 9;

    // Load first CSV
    let mut df = CsvReadOptions::default()
        .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
        .try_into_reader_with_file_path(Some("./data/lfs_csv/pub0106.csv".into()))
        .unwrap()
        .finish()
        .unwrap();

    // For all years
    for y in 2006..current_year + 1 {
        // For all months in each year
        for m in 1..12 + 1 {
            // skip the first one, since already loaded
            if y == 2006 && m == 1 {
                continue;
            }

            // Format name for the CSV file
            let csv_file = format!("./data/lfs_csv/pub{:02}{:02}.csv", m, y % 100);

            // stack new csv files on top
            df.vstack_mut(
                &CsvReadOptions::default()
                    .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
                    .try_into_reader_with_file_path(Some(csv_file.clone().into()))
                    .unwrap()
                    .finish()
                    .unwrap(),
            )?;

            // Print to keep track + size
            println!(
                "File: {} - Size: {}",
                csv_file,
                human_bytes(df.estimated_size() as f64)
            );

            // Recommended in the docs
            df.align_chunks();

            if y == current_year && m == current_month {
                break;
            }
        }
    }

    // Write large file as `lfs_large.csv`
    let mut file = std::fs::File::create("./data/lfs_large.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    Ok(())
}
