use mysql::{Column, Value, prelude::FromValue};
use std::sync::Arc;

use crate::importer::tablerow::TableRow;

/// A mock implementation of `TableRow`, for testing without a real database.
#[derive(Clone, Debug)]
pub struct MockTableRow {
    values: Vec<Value>,
    cols: Arc<[Column]>,
}

impl MockTableRow {
    /// Creates a new mock row from raw MySQL `Value`s and column metadata.
    pub fn new(values: Vec<Value>, columns: Vec<Column>) -> Self {
        Self {
            values,
            cols: Arc::from(columns),
        }
    }

    /// Creates a simplified mock row without column metadata.
    /// Useful when you only need the values for basic testing.
    pub fn from_values(values: Vec<Value>) -> Self {
        Self {
            values,
            cols: Arc::from(Vec::<Column>::new()),
        }
    }
}

impl TableRow for MockTableRow {
    fn get<T: FromValue>(&self, idx: usize) -> Option<T> {
        self.values
            .get(idx)
            .and_then(|v| mysql::from_value_opt::<T>(v.clone()).ok())
    }

    fn columns(&self) -> Arc<[Column]> {
        Arc::clone(&self.cols)
    }
}
