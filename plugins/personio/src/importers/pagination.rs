use model::BoxedError;

pub mod from;

pub struct PageResult<T> {
    pub data: T,
    pub total_pages: i32,
}

pub struct Paginator<T, P> {
    current_page: i32,
    total_pages: i32,
    limit: i32,
    fetcher: Box<dyn Fn(&P, i32, i32) -> Result<PageResult<T>, BoxedError>>,
}

impl<T, P> Paginator<T, P> {
    pub fn new<F>(limit: i32, fetcher: F) -> Self
    where
        F: Fn(&P, i32, i32) -> Result<PageResult<T>, BoxedError> + 'static,
    {
        Paginator {
            current_page: 1,
            total_pages: 1,
            limit,
            fetcher: Box::new(fetcher),
        }
    }

    /// Fetches all records over all pages
    pub fn fetch_all<C>(&mut self, params: &P, mut callback: C) -> Result<(), BoxedError>
    where
        C: FnMut(&T) -> Result<(), BoxedError>,
    {
        while self.current_page <= self.total_pages {
            let result = (self.fetcher)(params, self.limit, self.current_page)?;

            self.total_pages = result.total_pages;
            callback(&result.data)?;

            self.current_page += 1;
        }

        Ok(())
    }
}

pub fn next_offset(limit: i32, page: i32) -> i32 {
    (page - 1) * limit
}
