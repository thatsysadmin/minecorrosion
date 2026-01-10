use diesel::sql_types::Text;
use diesel_dynamic_schema::{table, Column, Table};

pub fn build_key_value_table(table_name: &String) -> KeyValue {
    let table = table(table_name);
    let key = table.column::<Text, _>("key");
    let value = table.column::<Text, _>("value");
    KeyValue {
        table,
        key,
        value,
    }
}

pub struct KeyValue<'a> {
    pub table: Table<&'a String, &'a String>,
    pub key: Column<Table<&'a String>, &'a str, Text>,
    pub value: Column<Table<&'a String>, &'a str, Text>
}