use model::{record::Record, BoxedError};

use crate::RecordHandler;

pub struct CollectingRecordHandler<'a> {
    records: &'a mut Vec<Record>,
}

impl<'a> CollectingRecordHandler<'a> {
    pub fn new(records: &'a mut Vec<Record>) -> Self {
        Self { records }
    }
}

impl<'a> RecordHandler for CollectingRecordHandler<'a> {
    fn handle_record(&mut self, record: &mut Record) -> Result<(), BoxedError> {
        self.records.push(Record::copy(record));
        Ok(())
    }
}

pub struct ClosureRecordHandler<F> {
    callback: F,
}

impl<F> ClosureRecordHandler<F>
where
    F: FnMut(&mut Record),
{
    pub fn new(callback: F) -> Self {
        Self { callback }
    }
}

impl<F> RecordHandler for ClosureRecordHandler<F>
where
    F: FnMut(&mut Record),
{
    fn handle_record(&mut self, record: &mut Record) -> Result<(), BoxedError> {
        (self.callback)(record);
        Ok(())
    }
}
