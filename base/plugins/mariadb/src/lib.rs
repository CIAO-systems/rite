mod exporter;
mod importer;

use model::{BoxedError, xml::common::DatabaseConnection};
use mysql::{Opts, Pool, PooledConn};

use crate::{exporter::MariaDBExporter, importer::MariaDBImporter};

/// This function creates an importer for data in a MariaDB/MySQL database
///
#[unsafe(no_mangle)]
pub fn create_importer(
    _name: &str,
) -> Result<Box<dyn model::import::Importer>, Box<dyn std::error::Error>> {
    Ok(Box::new(MariaDBImporter::new()))
}

/// This function creates an importer to write data in a MariaDB/MySQL database
///
#[unsafe(no_mangle)]
pub fn create_exporter(
    _name: &str,
) -> Result<Box<dyn model::export::Exporter>, Box<dyn std::error::Error>> {
    Ok(Box::new(MariaDBExporter::new()))
}

/// Create the MySQL connection string
pub fn create_connection_string(connection: &DatabaseConnection) -> String {
    format!(
        "mysql://{user}:{password}@{host}:{port}/{database}",
        host = connection.host,
        port = connection.port,
        user = connection.user,
        password = connection.password,
        database = connection.database
    )
}

/// Connect to a MariaDB/MySQL database
pub fn connect(connection: &DatabaseConnection) -> Result<PooledConn, BoxedError> {
    let connection_string = create_connection_string(connection);

    let pool = Pool::new(Opts::from_url(&connection_string)?)?;
    Ok(pool.get_conn()?)
}

#[cfg(test)]
mod tests {
    use model::xml::common::DatabaseConnection;

    use crate::{connect, create_connection_string, create_exporter, create_importer};

    fn type_of<T>(_: &T) -> &str {
        std::any::type_name::<T>()
    }

    #[test]
    fn test_create_importer() {
        let importer = create_importer("any");
        assert!(importer.is_ok());
        let importer = importer.unwrap();
        assert_eq!(
            type_of(&importer),
            "alloc::boxed::Box<dyn model::import::Importer>"
        );
    }

    #[test]
    fn test_create_exporter() {
        let exporter = create_exporter("any");
        assert!(exporter.is_ok());
        let exporter = exporter.unwrap();
        assert_eq!(
            type_of(&exporter),
            "alloc::boxed::Box<dyn model::export::Exporter>"
        );
    }

    #[test]
    fn test_create_connection_string() {
        let connection = DatabaseConnection {
            host: "host".into(),
            port: 73,
            database: "database".into(),
            user: "user".into(),
            password: "password".into(),
        };

        let connection_string = create_connection_string(&connection);
        assert_eq!(connection_string, "mysql://user:password@host:73/database");
    }

    #[test]
    fn test_connect() {
        let connection = DatabaseConnection {
            host: "host".into(),
            port: 73,
            database: "database".into(),
            user: "user".into(),
            password: "password".into(),
        };

        let result = connect(&connection);
        assert!(result.is_err());
        let e = result.err().unwrap().to_string();
        assert!(e.contains("host:73"));
    }
}
