use std::sync::Arc;

use mysql::{Column, Row, prelude::FromValue};

pub trait TableRow {
    fn get<T: FromValue>(&self, idx: usize) -> Option<T>;
    fn columns(&self) -> Arc<[Column]>;
}

impl TableRow for Row {
    fn get<T: FromValue>(&self, idx: usize) -> Option<T> {
        self.get::<T, usize>(idx)
    }

    fn columns(&self) -> Arc<[Column]> {
        self.columns()
    }
}
