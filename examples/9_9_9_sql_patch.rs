fn main() {}

// use connectorx::prelude::*;
// use polars::prelude::*;
// use std::convert::TryFrom;
// use std::mem::transmute;

// fn main() {
//     // Connect to PostgreSQL through the ConnectorX
//     let source_conn =
//         SourceConn::try_from("postgresql://postgres:postgres@localhost:5432").unwrap();

//     // Prepare query
//     let query = &[CXQuery::from("SELECT * FROM lfs")];

//     // ConnectorX query PostgreSQL and return arrow object
//     let arrow_obj = get_arrow(&source_conn, None, query)
//         .unwrap()
//         .arrow()
//         .unwrap();

//     let df = arrow_to_df(arrow_obj);

//     // Print table
//     println!("{}", df);

//     // Prepare query
//     let query = &[CXQuery::from(
//         "SELECT survmnth, avg(hrlyearn / 100) as avg_hourly FROM lfs where survyear = 2010 group by survmnth",
//     )];

//     // ConnectorX query PostgreSQL and return arrow object
//     let arrow_obj = get_arrow(&source_conn, None, query)
//         .unwrap()
//         .arrow()
//         .unwrap();

//     let df = arrow_to_df(arrow_obj);

//     // Print table
//     println!("{}", df);
// }

// fn arrow_to_df(arrow_obj: Vec<arrow::record_batch::RecordBatch>) -> DataFrame {
//     // The `.polars()`` (instead of `.arrow()`)from `ConnectorX` gives a Polars (version 0.32) object,
//     // when Polars (version 0.45) is the current version. Polars 0.32 has many missing features used
//     // in this book. Therefore, the below code convert the arrow-rs data from `ConnectorX` to polars-arrow
//     // (through ffi), then imports it to the current version of Polars. This is zero-copy.

//     // Get column names as Polars PlSmallStr
//     let names = arrow_obj[0]
//         .schema()
//         .fields()
//         .iter()
//         .map(|f| PlSmallStr::from(f.name()))
//         .collect::<Vec<PlSmallStr>>();

//     // Ready LazyFrame vector for the chunks
//     let mut lf_vec = vec![];

//     // The received arrow is chunked (for parallel processing) by ConnectorX (need to concat them later)
//     for batch in arrow_obj.into_iter() {
//         // Bach column vector
//         let mut columns = Vec::with_capacity(batch.num_columns());

//         // Arrow stores data by columns, therefore need to be Zero-copied by column
//         for (i, col) in batch.columns().iter().enumerate() {
//             // Convert to arrow_data::data::ArrayData (arrow-rs)
//             let array = col.to_data();

//             // Convert to ffi with arrow-rs
//             let (out_array, out_schema) = arrow::ffi::to_ffi(&array).unwrap();

//             // Import field from ffi with polars
//             let field = unsafe {
//                 polars_arrow::ffi::import_field_from_c(transmute::<
//                     &arrow::ffi::FFI_ArrowSchema,
//                     &polars_arrow::ffi::ArrowSchema,
//                 >(&out_schema))
//             }
//             .unwrap();

//             // Import data from ffi with polars
//             let data = unsafe {
//                 polars_arrow::ffi::import_array_from_c(
//                     transmute::<arrow::ffi::FFI_ArrowArray, polars_arrow::ffi::ArrowArray>(
//                         out_array,
//                     ),
//                     field.dtype().clone(),
//                 )
//             }
//             .unwrap();

//             // Create Polars series from arrow column
//             columns.push(Series::from_arrow(names[i].clone(), data).unwrap());
//         }

//         // Create DataFrame from the columns
//         lf_vec.push(DataFrame::from_iter(columns).lazy());
//     }

//     // Concat the chunks
//     let union_args = UnionArgs::default();
//     concat(lf_vec, union_args).unwrap().collect().unwrap()
// }
