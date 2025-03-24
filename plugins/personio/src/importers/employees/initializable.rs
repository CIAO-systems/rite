use model::{BoxedError, Initializable};
use personio_rs::auth::login;
use tokio::runtime::Runtime;

use super::FLAG_SALARY;

const CFG_CLIENT_ID: &str = "client_id";
const CFG_CLIENT_SECRET: &str = "client_secret";
const CFG_OPTIONS_LIMIT: &str = "options.limit";

impl Initializable for super::Employees {
    fn init(
        &mut self,
        config: Option<model::xml::config::Configuration>,
    ) -> Result<(), BoxedError> {
        if let Some(config) = config {
            if let Some(client_id) = config.get(CFG_CLIENT_ID) {
                if let Some(client_secret) = config.get(CFG_CLIENT_SECRET) {
                    let runtime = Runtime::new()?;
                    let result: Result<String, BoxedError> =
                        runtime.block_on(async { Ok(login(client_id, client_secret).await?) });
                    match result {
                        Ok(token) => {
                            // We have a valid token now, store it and the tokio runtime
                            self.token = Some(token);
                            self.runtime = Some(runtime);
                        }
                        Err(e) => return Err(e),
                    }
                }
            }

            // read flags
            if let Some(salary) = config.get(FLAG_SALARY) {
                if let Ok(salary) = salary.parse::<bool>() {
                    self.flags.insert(String::from(FLAG_SALARY), salary);
                }
            }

            // read options
            if let Some(limit) = config.get(CFG_OPTIONS_LIMIT) {
                if let Ok(limit) = limit.parse::<i32>() {
                    self.limit = Some(limit);
                }
            }
        }
        Ok(())
    }
}
