use protobuf::well_known_types::{ListValue, Value};
use protobuf::RepeatedField;

use google::spanner::v1::keys;

/// Identify range of table rows via start/end points.
#[derive(Clone)]
pub struct KeyRange {
    start_open: Vec<String>,
    start_closed: Vec<String>,
    end_open: Vec<String>,
    end_closed: Vec<String>
}

impl KeyRange {
    /// Initializes a new KeyRange.
    ///
    /// Specify either a `start_open` or `start_closed` key, or
    /// defaults to `start_closed = Vec::new();`. Specify either
    /// an `end_open` or `end_closed` key, or defaults to
    /// `end_closed = Vec::new();`. However, at least one key
    /// has to be specified. If no keys are specified, it'll panic.
    ///
    /// # Arguments
    ///
    /// * `start_open` - Keys identifying start of range (this key excluded).
    ///
    /// * `start_closed` - Keys identifying start of range (this key included).
    ///
    /// * `end_open` - Keys identifying end of range (this key excluded).
    ///
    /// * `end_closed` - Keys identifyind end of range (this key included).
    ///
    /// # Return value
    ///
    /// A new `KeyRange`.
    ///
    /// # Panics
    ///
    /// * If no keys are specified.
    /// * If both `start_open`/`start_closed` are specified.
    /// * If both `end_open`/`end_closed` are specified.
    pub fn new(start_open: Option<Vec<String>>, start_closed: Option<Vec<String>>, end_open: Option<Vec<String>>, end_closed: Option<Vec<String>>) -> Self {
        let so = start_open.unwrap_or(Vec::new());
        let sc = start_closed.unwrap_or(Vec::new());
        let eo = end_open.unwrap_or(Vec::new());
        let ec = end_closed.unwrap_or(Vec::new());
        if so.is_empty() && sc.is_empty() && ec.is_empty() && eo.is_empty() {
            panic!("Must specify at least a start or end row");
        }

        if !so.is_empty() && !sc.is_empty() {
            panic!("Specify one of 'start_open'/'start_closed'.");
        }

        if !eo.is_empty() && !ec.is_empty() {
            panic!("Specify one of 'end_open'/'start_closed'.");
        }

        KeyRange {
            start_open: so,
            start_closed: sc,
            end_open: eo,
            end_closed: ec
        }
    }

    /// Construct a KeyRange protobuf.
    ///
    /// # Return value
    ///
    /// Protobuf corresponding to this instance.
    pub fn to_pb(&self) -> keys::KeyRange {
        let mut kr = keys::KeyRange::new();
        if !self.start_open.is_empty() {
            kr.set_start_open(make_list_value_pb(&self.start_open));
        }

        if !self.start_closed.is_empty() {
            kr.set_start_closed(make_list_value_pb(&self.start_closed));
        }

        if !self.end_open.is_empty() {
            kr.set_end_open(make_list_value_pb(&self.end_open));
        }

        if !self.end_closed.is_empty() {
            kr.set_end_closed(make_list_value_pb(&self.end_closed));
        }

        kr
    }
}

/// Identify table rows via keys/ranges.
#[derive(Clone)]
pub struct KeySet {
    keys: Vec<Vec<String>>,
    ranges: Vec<KeyRange>,
    all: bool
}

impl KeySet {
    /// Initializes a new KeySet.
    ///
    /// # Arguments
    ///
    /// * `keys` - Keys identifying individual rows withing a table.
    ///
    /// * `ranges` - Ranges identifying rows within a table.
    ///
    /// * `all` - If True, identfy all rows within a table.
    ///
    /// # Return value
    ///
    /// A new `KeySet`.
    pub fn new(keys: Option<Vec<String>>, ranges: Option<Vec<KeyRange>>, all: Option<bool>) -> Self {
        let k = keys.unwrap_or(Vec::new());
        let r = ranges.unwrap_or(Vec::new());
        let a = all.unwrap_or(false);
        if a && (!k.is_empty() || !r.is_empty()) {
            panic!("'all' is exclusive of 'keys'/'ranges'.");
        }
        KeySet {
            keys: vec![k],
            ranges: r,
            all: a
        }
    }

    /// Construct a KeySet protobuf.
    ///
    /// # Return value
    ///
    /// Protobuf corresponding to this instance.
    pub fn to_pb(&self) -> keys::KeySet {
        let mut r = keys::KeySet::new();
        r.set_all(self.all);
        
        if !self.ranges.is_empty() {
            r.set_ranges(RepeatedField::from_vec(self.make_ranges_to_pb()));
        }

        if !self.keys.is_empty() {
            r.set_keys(make_list_value_pbs(&self.keys));
        }

        r
    }

    /// Helper method for `to_pb`
    fn make_ranges_to_pb(&self) -> Vec<keys::KeyRange> {
        self.ranges.iter().map(|kr| kr.to_pb()).collect()
    }
}

/// Helper method for `to_pb`
fn make_list_value_pb(values: &Vec<String>) -> ListValue {
    let mut lv = ListValue::new();
    let a = values.iter().map(|v| {
        make_value(v.to_string())
    }).collect();
    lv.set_values(RepeatedField::from_vec(a));
    lv
}

/// Helper method for `to_pb`
fn make_list_value_pbs(values: &Vec<Vec<String>>) -> RepeatedField<ListValue> {
    let a = values.iter().map(|v| {
        make_list_value_pb(v)
    }).collect();
    RepeatedField::from_vec(a)
}

/// Helper method for `to_pb`
fn make_value(value: String) -> Value {
    let mut v = Value::new();
    v.set_string_value(value);
    v
}