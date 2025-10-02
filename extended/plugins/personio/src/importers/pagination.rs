use model::BoxedError;

pub mod parameters;
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

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use super::*;

    #[derive(Clone, Debug, PartialEq)]
    struct TestData {
        value: i32,
    }

    #[derive(Debug, PartialEq)]
    struct TestParams {
        start: i32,
    }

    #[test]
    fn test_next_offset() {
        assert_eq!(next_offset(10, 1), 0);
        assert_eq!(next_offset(10, 2), 10);
        assert_eq!(next_offset(10, 3), 20);
        assert_eq!(next_offset(5, 4), 15);
    }

    #[test]
    fn test_paginator_fetch_all_single_page() {
        let params = TestParams { start: 0 };
        let fetcher = |p: &TestParams,
                       limit: i32,
                       page: i32|
         -> Result<PageResult<Vec<TestData>>, BoxedError> {
            assert_eq!(p.start, 0);
            assert_eq!(limit, 10);
            assert_eq!(page, 1);
            Ok(PageResult {
                data: vec![TestData { value: 1 }, TestData { value: 2 }],
                total_pages: 1,
            })
        };

        let mut paginator = Paginator::new(10, fetcher);

        let mut collected_data = Vec::new();
        let callback = |data: &Vec<TestData>| -> Result<(), BoxedError> {
            collected_data.extend_from_slice(data);
            Ok(())
        };

        paginator.fetch_all(&params, callback).unwrap();

        assert_eq!(
            collected_data,
            vec![TestData { value: 1 }, TestData { value: 2 }]
        );
    }

    #[test]
    fn test_paginator_fetch_all_multiple_pages() {
        let params = TestParams { start: 0 };
        let page_count = Mutex::new(1); // Use Mutex to share mutable state
        let fetcher = move |p: &TestParams,
                            limit: i32,
                            page: i32|
              -> Result<PageResult<Vec<TestData>>, BoxedError> {
            let mut count = page_count.lock().unwrap(); // Lock the Mutex
            assert_eq!(p.start, 0);
            assert_eq!(limit, 1);
            assert_eq!(page, *count);

            let data = vec![TestData { value: *count }];
            let total_pages = 3;

            *count += 1; // Increment the page count
            Ok(PageResult { data, total_pages })
        };

        let mut paginator = Paginator::new(1, fetcher);

        let mut collected_data = Vec::new();
        let callback = |data: &Vec<TestData>| -> Result<(), BoxedError> {
            collected_data.extend_from_slice(data);
            Ok(())
        };

        paginator.fetch_all(&params, callback).unwrap();

        assert_eq!(
            collected_data,
            vec![
                TestData { value: 1 },
                TestData { value: 2 },
                TestData { value: 3 },
            ]
        );
    }

    #[test]
    fn test_paginator_fetch_all_error_in_fetcher() {
        let params = TestParams { start: 0 };
        let fetcher =
            |_: &TestParams, _: i32, _: i32| -> Result<PageResult<Vec<TestData>>, BoxedError> {
                Err(BoxedError::from("Fetch error"))
            };

        let mut paginator = Paginator::new(1, fetcher);

        let callback = |_data: &Vec<TestData>| -> Result<(), BoxedError> { Ok(()) };

        let result = paginator.fetch_all(&params, callback);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Fetch error");
    }

    #[test]
    fn test_paginator_fetch_all_error_in_callback() {
        let params = TestParams { start: 0 };
        let fetcher =
            |_: &TestParams, _: i32, _: i32| -> Result<PageResult<Vec<TestData>>, BoxedError> {
                Ok(PageResult {
                    data: vec![TestData { value: 1 }],
                    total_pages: 1,
                })
            };

        let mut paginator = Paginator::new(1, fetcher);

        let callback = |_data: &Vec<TestData>| -> Result<(), BoxedError> {
            Err(BoxedError::from("Callback error"))
        };

        let result = paginator.fetch_all(&params, callback);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().to_string(), "Callback error");
    }
}
