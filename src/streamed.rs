use google::spanner::v1::result_set::{ResultSetStats, PartialResultSet, ResultSetMetadata};
use google::spanner::v1::type_pb::{Type, TypeCode, StructType_Field};

use snapshot::Snapshot;

use grpcio::ClientSStreamReceiver;
use protobuf::well_known_types::{Value, ListValue};
use protobuf::RepeatedField;
use futures::{Future, Stream};
use futures::stream::{StreamFuture};

/// Process a sequence of partial result sets into a single set of row data.
pub struct StreamedResultSet<'a> {
    response_stream: Option<StreamFuture<ClientSStreamReceiver<PartialResultSet>>>,
    rows: Vec<Vec<Value>>,
    counter: i32,
    metadata: Option<ResultSetMetadata>,
    stats: Option<ResultSetStats>,
    current_row: Vec<Value>,
    pending_chunk: Option<Value>,
    source: Option<&'a mut Snapshot<'a>>
}

impl<'a> StreamedResultSet<'a> {
    /// Initializes a new StreamedResultSet.
    ///
    /// # Arguments
    ///
    /// * `response_stream` - A [`StreamFuture`] result from API calls.
    ///
    /// * `source` - A [`Snapshot`] from which the result set was fetched.
    ///
    /// # Return value
    ///
    /// A new `StreamedResultSet`.
    ///
    /// [`StreamFuture`]: https://docs.rs/futures/0.2.1/futures/stream/struct.StreamFuture.html
    /// [`Snapshot`]: ../snapshot/struct.Snapshot.html
    pub fn new(response_stream: StreamFuture<ClientSStreamReceiver<PartialResultSet>>, source: Option<&'a mut Snapshot<'a>>) -> Self {
        StreamedResultSet {
            response_stream: Some(response_stream),
            rows: Vec::new(),
            counter: 0,
            metadata: None,
            stats: None,
            current_row: Vec::new(),
            pending_chunk: None,
            source: source
        }
    }

    /// Field descriptors for result set columns
    ///
    /// # Return value
    ///
    /// A list of fields describing column names/types.
    pub fn fields(&self) -> &[StructType_Field] {
        match self.metadata {
            Some(ref m) => { m.get_row_type().get_fields() },
            None => { &[] }
        }
    }

    /// Result set metadata
    ///
    /// # Return value
    ///
    /// Structure describing the results
    pub fn metadata(&self) -> Option<&ResultSetMetadata> {
        self.metadata.as_ref()
    }

    /// Result set statistics.
    ///
    /// # Return value
    ///
    /// Structure describing status about the response.
    pub fn stats(&self) -> Option<&ResultSetStats> {
        self.stats.as_ref()
    }

    /// Merge pending chunk with the next value.
    ///
    /// # Arguments
    ///
    /// * `value` - Continuation of chunked value from previous
    ///             partial result set.
    ///
    /// # Return value
    ///
    /// The merge value
    fn merge_chunk(&self, value: &Value) -> Value {
        let current_column = self.current_row.len();
        let field = &self.fields()[current_column];
        let merged = match self.pending_chunk {
            Some(ref pc) => { merge_by_type(pc, value, field.get_field_type()) },
            None => { Value::new() }
        };
        merged
    }

    /// Merge values into rows
    ///
    /// # Arguments
    /// 
    /// * `values` - Non-chunked values from partial result set.
    fn merge_values(&mut self, values: Vec<Value>) {
        let width = self.fields().len();
        for value in values {
            self.current_row.push(value);
            if self.current_row.len() == width {
                self.rows.push(self.current_row.clone());
                self.current_row = Vec::new();
            }
        }
    }

    /// Consume the next partial result set from the stream.
    ///
    /// Parse the result set into new/existing rows in `rows`
    fn consume_next(&mut self) -> bool {
        let stream = self.response_stream.take().unwrap();
        match stream.wait() {
            Ok((Some(mut prs), s)) => {
                self.response_stream = Some(s.into_future());
                self.counter += 1;
                if self.metadata.is_none() {
                    self.metadata = Some(prs.take_metadata());

                    match self.source {
                        Some(ref mut s) => {
                            if s.transaction_id.is_none() {
                                match self.metadata {
                                    Some(ref m) => { s.set_transaction_id(m.get_transaction().get_id().to_vec()); },
                                    None => { }
                                }
                            }
                        },
                        None => { }
                    }
                }

                if prs.has_stats() {
                    self.stats = Some(prs.take_stats());
                }

                let mut values = prs.get_values().to_vec();
                if self.pending_chunk.is_some() {
                    values[0] = self.merge_chunk(&values[0]);
                }

                if prs.get_chunked_value() {
                    self.pending_chunk = values.pop();
                }

                self.merge_values(values);
                return true
            }
            Ok((None, _)) => false,
            Err((e, _)) => panic!("Consume next row failed: {:?}", e),
        }
    }

    /// Return exactly one result or panic.
    ///
    /// # Panics
    ///
    /// * If there no result.
    /// * If there are multiple results.
    /// * If consumption has already ocurred, in whole or in part.
    pub fn one(&mut self) -> Vec<Value> {
        let answer = self.one_or_none();
        match answer {
            Err(m) => { panic!(m.to_string()) },
            Ok(v) => {
                if v.is_empty() {
                    panic!("No rows matched the given query.");
                } else {
                    v
                }
            }
        }
    }

    /// Return exactly one result, or None if there are no results.
    ///
    /// # Panics
    ///
    /// If there multiple results.
    /// If consumption has already ocurred, in whole or in part.
    pub fn one_or_none(&mut self) -> Result<Vec<Value>, &str> {
        if self.metadata.is_some() {
            return Err("Can not call '.one' or '.one_or_none' after stream consumption has already started.")
        }

        let answer = self.next();

        if answer.is_none() {
            return Ok(Vec::new())
        }

        let other_answer = self.next();
        if other_answer.is_some() {
            Err("Expected one result; got more")
        } else {
            Ok(answer.unwrap())
        }
    }
}

impl<'a> Iterator for StreamedResultSet<'a> {
    type Item = Vec<Value>;

    fn next(&mut self) -> Option<Self::Item> {
        while self.rows.is_empty() {
            if !self.consume_next() {
                return None
            }
        }
        self.rows.pop()
    }
}

/// Helper for `merge_chunk`
fn merge_by_type (lhs: &Value, rhs: &Value, typed: &Type) -> Value {
    match typed.get_code() {
        TypeCode::BYTES |
        TypeCode::DATE |
        TypeCode::INT64 |
        TypeCode::STRING |
        TypeCode::TIMESTAMP |
        TypeCode::FLOAT64 => { merge_string(lhs, rhs) },
        TypeCode::ARRAY => { merge_array(lhs, rhs, typed) },
        TypeCode::STRUCT => { merge_struct(lhs, rhs, typed) },
        TypeCode::BOOL |
        TypeCode::TYPE_CODE_UNSPECIFIED => { panic!("Unmeargeable type"); }
    }
}

/// Helper for `merge_by_type`
fn merge_string (lhs: &Value, rhs: &Value) -> Value {
    let mut v = Value::new();
    v.set_string_value(lhs.get_string_value().to_string() + rhs.get_string_value());
    v
}

/// Helper for `merge_by_type`
fn merge_array (lhs: &Value, rhs: &Value, typed: &Type) -> Value {
    let element_type = typed.get_array_element_type();
    let mut l = lhs.get_list_value().get_values().to_vec();
    let mut r = rhs.get_list_value().get_values().to_vec();
    
    if element_type.get_code() == TypeCode::BOOL || (l.is_empty() || r.is_empty()) {
        l.append(&mut r);
        let mut v = Value::new();
        let mut lv = ListValue::new();
        lv.set_values(RepeatedField::from_vec(l));
        v.set_list_value(lv);
        return v
    }

    let first = r.remove(0);
    if first.has_null_value() {
        l.push(first);
    } else {
        let last = l.pop();
        if element_type.get_code() != TypeCode::BOOL {
            match last {
                Some(ref v) => {
                    l.push(merge_by_type(v, &first, element_type));
                },
                None => { }
            };
        } else {
            l.push(last.unwrap());
            l.push(first);
        }
    }
    let mut v = Value::new();
    let mut list = ListValue::new();
    l.append(&mut r);
    list.set_values(RepeatedField::from_vec(l));
    v.set_list_value(list);
    v
}

/// Helper for `merge_by_type`
fn merge_struct (lhs: &Value, rhs: &Value, typed: &Type) -> Value {
    let fields = typed.get_struct_type().get_fields();
    let mut l = lhs.get_list_value().get_values().to_vec();
    let mut r = rhs.get_list_value().get_values().to_vec();

    let candidate_type = fields[l.len() - 1].get_field_type();
    let first = r.pop();
    if first.is_some() {
        let f = first.unwrap();
        if f.has_null_value() || candidate_type.get_code() == TypeCode::BOOL {
            l.push(f);
        } else {
            let last = l.pop();
            match last {
                Some(ref v) => {
                    l.push(merge_by_type(v, &f, candidate_type));
                },
                None => { }
            };
        }
    }
    let mut v = Value::new();
    let mut list = ListValue::new();
    l.append(&mut r);
    list.set_values(RepeatedField::from_vec(l));
    v.set_list_value(list);
    v
}
