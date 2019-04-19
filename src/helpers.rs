use google::spanner::v1::mutation::Mutation_Write;
use google::spanner::v1::type_pb;
use protobuf::well_known_types::{Timestamp, Duration, Value, ListValue};
use chrono::{DateTime, NaiveDateTime, Utc};
use protobuf::{RepeatedField, ProtobufEnum};
use std::collections::HashMap;

pub fn pb_timestamp_to_datetime(timestamp_pb: Timestamp) -> DateTime<Utc> {
    DateTime::from_utc(NaiveDateTime::from_timestamp(timestamp_pb.get_seconds(), timestamp_pb.get_nanos() as u32), Utc)
}

pub fn datetime_to_pb_timestamp(datetime: DateTime<Utc>) -> Timestamp {
    let mut ts = Timestamp::new();
    ts.set_seconds(datetime.timestamp() as i64);
    ts.set_nanos(datetime.timestamp_subsec_nanos() as i32);
    ts
}

pub fn datetime_to_duration_pb(datetime: DateTime<Utc>) -> Duration {
    let mut d = Duration::new();
    d.set_seconds(datetime.timestamp());
    d.set_nanos(datetime.timestamp_subsec_nanos() as i32);
    d
}

pub enum QueryMode {
    NORMAL = 0,
    PLAN = 1,
    PROFILE = 2,
}

#[derive(Clone)]
#[warn(non_camel_case_types)]
pub enum Type {
    TypeCodeUnspecified = 0,
    BOOL = 1,
    INT64 = 2,
    FLOAT64 = 3,
    TIMESTAMP = 4,
    DATE = 5,
    STRING = 6,
    BYTES = 7,
    ARRAY = 8,
    STRUCT = 9,
}

/// Helper for method [`Batch.insert`] et aliae.
///
/// # Arguments
///
/// * `table` - Name of table to be modified.
///
/// * `columns` - Name of table columns to be modified.
///
/// * `values` - Values to be modified.
///
/// # Return value
///
/// A write protobuf
///
/// [`Batch.insert`]: ../batch/struct.Batch.html#method.insert
pub fn make_write_pb(table: String, columns: Vec<String>, values: Vec<Vec<Value>>) -> Mutation_Write {
    let mut m = Mutation_Write::new();
    m.set_table(table);
    m.set_columns(RepeatedField::from_vec(columns));
    let lvs = values.iter().map(|v| {
        let mut lv = ListValue::new();
        lv.set_values(RepeatedField::from_vec(v.to_vec()));
        lv
    }).collect();
    m.set_values(RepeatedField::from_vec(lvs));
    m
}

pub fn make_type_pb_map(types: HashMap<String, Type>) -> HashMap<String, type_pb::Type> {
    let mut result = HashMap::new();
    for (key, value) in &types {
        let mut v = type_pb::Type::new();
        v.set_code(type_pb::TypeCode::from_i32(value.clone() as i32).unwrap());
        result.insert(key.to_string(), v);
    }
    result
}