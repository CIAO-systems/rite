use std::pin::Pin;

use futures::Stream;
use tonic::{Request, Response, Status};

use crate::com::atoss::atc::protobuf::{
    absences_service_server::AbsencesService, data_set_service_server::DataSetService, Absence,
    AbsencesRequest, Filter, Record,
};
pub struct MockDataSetService;
pub struct MockAbsenceService;

#[tonic::async_trait]
impl DataSetService for MockDataSetService {
    type getStream = Pin<Box<dyn Stream<Item = Result<Record, Status>> + Send + 'static>>;

    async fn get(&self, _request: Request<Filter>) -> Result<Response<Self::getStream>, Status> {
        // Create fake records based on filter
        let records = vec![];

        // Turn Vec<Record> into a stream of Result<Record, Status>
        let output = tokio_stream::iter(records.into_iter().map(|r| Ok(r)));

        Ok(Response::new(Box::pin(output) as Self::getStream))
    }
}

#[tonic::async_trait]
impl AbsencesService for MockAbsenceService {
    type getSingleDayAbsencesStream =
        Pin<Box<dyn Stream<Item = Result<Absence, Status>> + Send + 'static>>;

    async fn get_single_day_absences(
        &self,
        _request: Request<AbsencesRequest>,
    ) -> Result<Response<Self::getSingleDayAbsencesStream>, Status> {
        // mock response stream
        let absences = vec![];

        let output = tokio_stream::iter(absences.into_iter().map(|a| Ok(a)));

        Ok(Response::new(Box::pin(output)))
    }
}
