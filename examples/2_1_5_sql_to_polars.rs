use polars::{datatypes, prelude::*};
use polars_arrow::array;
use postgres::{types::Type, Client, NoTls};

fn main() {
    let mut client = Client::connect("host=localhost user=postgres", NoTls).unwrap();

    let rows = client.query("select * from lfs", &[]).unwrap();

    // Create series from the vectors
    let mut fields: Vec<(String, datatypes::DataType)> = Vec::new();
    let first_row = &rows[0];
    let column_count = first_row.len();
    for i in 0..column_count {
        let name = first_row.columns()[i].name().to_string();
        let data_type: datatypes::DataType = match *first_row.columns()[i].type_() {
            Type::INT2 => datatypes::DataType::Int16,
            Type::INT4 => datatypes::DataType::Int32,
            Type::INT8 => datatypes::DataType::Int64,
            _ => panic!("Add more data types {}", first_row.columns()[i].type_()),
        };
        fields.push((name, data_type));
    }

    let mut arrow_arrays: Vec<Vec<ArrayRef>> = vec![];

    for (col_index, column) in first_row.columns().iter().enumerate() {
        let mut array_data: Vec<ArrayRef> = vec![];

        for (_row_index, row) in rows.iter().enumerate() {
            let array: ArrayRef = match *column.type_() {
                Type::INT2 => Box::new(array::Int16Array::from(vec![
                    match row.try_get(col_index) {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    },
                ])),
                Type::INT4 => Box::new(array::Int32Array::from(vec![
                    match row.try_get(col_index) {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    },
                ])),
                Type::INT8 => Box::new(array::Int64Array::from(vec![
                    match row.try_get(col_index) {
                        Ok(val) => Some(val),
                        Err(_) => None,
                    },
                ])),
                // Add more cases for other PostgreSQL types as needed
                _ => panic!("Add more data types {}", column.type_()),
            };

            array_data.push(array);
        }

        arrow_arrays.push(array_data);
    }

    let mut series: Vec<Column> = vec![];

    for (array, field) in arrow_arrays.iter().zip(fields.iter()) {
        unsafe {
            let s = Column::new(
                PlSmallStr::from(&field.0),
                Series::from_chunks_and_dtype_unchecked(
                    PlSmallStr::from(&field.0),
                    array.to_vec(),
                    &field.1,
                ),
            );

            series.push(s);
        }
    }

    let df = DataFrame::new(series).unwrap();

    println!("{}", df);
}
