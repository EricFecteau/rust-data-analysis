// === imports
use polars::prelude::*;

// === main
fn main() {
    // === program

    // Data File

    // Read CSV
    let lf = LazyCsvReader::new(PlPath::from_string("./data/raw/census.csv".to_string()))
        .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
        .with_has_header(true)
        .finish()
        .unwrap();

    // Rename columns
    let lf = lf.select([
        col("resident_id_m").alias("id"),
        col("approx_social_grade").alias("social"),
        col("country_of_birth_3a").alias("birth"),
        col("economic_activity_status_10m").alias("econ"),
        col("ethnic_group_tb_6a").alias("ethnic"),
        col("health_in_general").alias("health"),
        col("hh_families_type_6a").alias("fam_type"),
        col("hours_per_week_worked").alias("hours_worked"),
        col("in_full_time_education").alias("education"),
        col("industry_10a").alias("industry"),
        col("iol22cd").alias("london"),
        col("legal_partnership_status_6a").alias("mar_stat"),
        col("occupation_10a").alias("occupation"),
        col("region"),
        col("religion_tb").alias("religion"),
        col("residence_type"),
        col("resident_age_7d").alias("age_group"),
        col("sex"),
        col("usual_short_student").alias("keep_type"),
    ]);

    // Write output to CSV
    let mut df = lf.collect().unwrap();
    let mut file = std::fs::File::create("./data/raw/census.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    // Codeset

    // Read CSV
    let lf = LazyCsvReader::new(PlPath::from_string(
        "./data/codeset/codeset.csv".to_string(),
    ))
    .with_infer_schema_length(Some(10_000)) // Default 100, missing = String
    .with_has_header(true)
    .finish()
    .unwrap();

    // Rename variables
    let lf = lf.with_column(col("variable").replace_strict(
        lit(Series::from_iter(vec![
            "resident_id_m",
            "approx_social_grade",
            "country_of_birth_3a",
            "economic_activity_status_10m",
            "ethnic_group_tb_6a",
            "health_in_general",
            "hh_families_type_6a",
            "hours_per_week_worked",
            "in_full_time_education",
            "industry_10a",
            "iol22cd",
            "legal_partnership_status_6a",
            "occupation_10a",
            "region",
            "religion_tb",
            "residence_type",
            "resident_age_7d",
            "sex",
            "usual_short_student",
        ])),
        lit(Series::from_iter(vec![
            "id",
            "social",
            "birth",
            "econ",
            "ethnic",
            "health",
            "fam_type",
            "hours_worked",
            "education",
            "industry",
            "london",
            "mar_stat",
            "occupation",
            "region",
            "religion",
            "residence_type",
            "age_group",
            "sex",
            "keep_type",
        ])),
        None,
        Some(DataType::String),
    ));

    // Write output to CSV
    let mut df = lf.collect().unwrap();
    let mut file = std::fs::File::create("./data/codeset/codeset.csv").unwrap();
    CsvWriter::new(&mut file).finish(&mut df).unwrap();

    // === end
}
