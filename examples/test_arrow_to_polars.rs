use connectorx::prelude::*;
use polars::prelude::*;
use polars_arrow::ffi;
use std::convert::TryFrom;
use std::mem::transmute;

fn main() {
    let source_conn =
        SourceConn::try_from("postgresql://postgres:postgres@localhost:5432").unwrap();
    let queries = &[CXQuery::from("SELECT * FROM lfs")];
    let destination = get_arrow(&source_conn, None, queries).unwrap();

    // The .polars() from `connectorx` gives a Polars (version 0.32) object, when Polars (version 0.45)
    // is the current version. 0.32 has many missing features used in this book. Therefore, the below code
    // convert the arrow-rs data from `connectorx` to polars-arrow (through ffi), then imports it to the
    // current version of Polars. This copies data exactly once.

    let arrow = destination.arrow().unwrap();

    let names = arrow[0]
        .schema()
        .fields()
        .iter()
        .map(|f| PlSmallStr::from(f.name()))
        .collect::<Vec<PlSmallStr>>();

    let mut lf_vec = vec![];

    for batch in arrow.into_iter() {
        // Bach column vector
        let mut columns = Vec::with_capacity(batch.num_columns());

        for (i, col) in batch.columns().iter().enumerate() {
            // Convert to data
            let array = col.to_data();

            // Convert to ffi with arrow-rs
            let (out_array, out_schema) = arrow::ffi::to_ffi(&array).unwrap();

            // Import field from ffi with polars
            let field = unsafe {
                ffi::import_field_from_c(transmute::<
                    &arrow::ffi::FFI_ArrowSchema,
                    &polars_arrow::ffi::ArrowSchema,
                >(&out_schema))
            }
            .unwrap();

            // Import data from ffi with polars
            let data = unsafe {
                ffi::import_array_from_c(
                    transmute::<arrow::ffi::FFI_ArrowArray, polars_arrow::ffi::ArrowArray>(
                        out_array,
                    ),
                    field.dtype().clone(),
                )
            }
            .unwrap();

            columns.push(Series::from_arrow(names[i].clone(), data).unwrap());
        }

        lf_vec.push(DataFrame::from_iter(columns).lazy());
    }

    let union_args = UnionArgs::default();
    let lf = concat(lf_vec, union_args).unwrap();

    println!("{}", lf.collect().unwrap())
}
