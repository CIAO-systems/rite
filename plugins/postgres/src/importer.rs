use config::RitePostgresImport;
use import::Importer;
use model::{
    field::Field,
    record::Record,
    value::Value,
    xml::{self, file::load_and_substitute_from_env},
    Initializable,
};

mod config;

#[derive(Debug)]
pub struct PostgresImporter {
    postgres: Option<RitePostgresImport>,
}

impl PostgresImporter {
    pub(crate) fn new() -> Self {
        Self { postgres: None }
    }
}

impl Initializable for PostgresImporter {
    fn init(
        &mut self,
        config: Option<xml::config::Configuration>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(config) = config {
            if let Some(ref xml) = config.xml {
                match load_and_substitute_from_env(xml, &std::collections::HashMap::new()) {
                    Ok(xml_contents) => {
                        let postgres: config::RitePostgresImport =
                            match serde_xml_rs::from_str(&xml_contents) {
                                Ok(x) => x,
                                Err(e) => {
                                    return Err(format!(
                                        "Cannot parse contents from {}: {}",
                                        xml, e
                                    )
                                    .into())
                                }
                            };
                        self.postgres = Some(postgres);
                    }
                    Err(e) => return Err(e.into()),
                }
            }
        }
        Ok(())
    }
}

impl Importer for PostgresImporter {
    fn read(
        &mut self,
        callback: &mut dyn FnMut(model::record::Record),
    ) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ref postgres) = self.postgres {
            // connect to database
            // execute query
            let connection_string = format!(
                "host={} port={} user={} password={} dbname={}",
                postgres.connection.host,
                postgres.connection.port,
                postgres.connection.user,
                postgres.connection.password,
                postgres.connection.database
            );

            let mut client = postgres::Client::connect(&connection_string, postgres::NoTls)?;
            // Execute the query
            let rows = client.query(&postgres.sql, &[])?;

            // convert each row to a Record and send it to the callback
            for row in rows {
                let record = handle_row(row)?;
                callback(record);
            }
        }

        Ok(())
    }

    fn reset(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        panic!("Not implemented");
    }
}

fn handle_row(row: postgres::Row) -> Result<Record, Box<dyn std::error::Error>> {
    let mut record = Record::new();
    for (idx, column) in row.columns().iter().enumerate() {
        let field_type = column.type_().name();
        match field_type {
            "int4" => {
                let value: i32 = row.get(idx);
                record.fields_as_mut().push(Field::new_value(
                    column.name().to_string(),
                    Value::I32(value),
                ));
            }
            "int8" => {
                let value: i64 = row.get(idx);
                record.fields_as_mut().push(Field::new_value(
                    column.name().to_string(),
                    Value::I64(value),
                ));
            }
            "text" | "varchar" => {
                let value: String = row.get(idx);
                record.fields_as_mut().push(Field::new_value(
                    column.name().to_string(),
                    Value::String(value),
                ));
            }
            "bool" => {
                let value: bool = row.get(idx);
                record.fields_as_mut().push(Field::new_value(
                    column.name().to_string(),
                    Value::Bool(value),
                ));
            }
            "float4" => {
                let value: f32 = row.get(idx);
                record.fields_as_mut().push(Field::new_value(
                    column.name().to_string(),
                    Value::F32(value),
                ));
            }
            "float8" => {
                let value: f64 = row.get(idx);
                record.fields_as_mut().push(Field::new_value(
                    column.name().to_string(),
                    Value::F64(value),
                ));
            }
            _ => return Err(format!("Unsupported type: {}", field_type).into()),
        }
    }
    Ok(record)
}

#[cfg(test)]
mod tests {

    use import::Importer;
    use model::{xml, Initializable};

    use super::PostgresImporter;

    #[test]
    fn test_import() -> Result<(), Box<dyn std::error::Error>> {
        let mut importer = PostgresImporter::new();
        let config = xml::config::Configuration::with_xml("../../data/postgres-import-config.xml");

        importer.init(Some(config))?;

        let mut count = 0;
        importer.read(&mut |_record| {
            count = count + 1;
        })?;

        assert!(count > 0);
        Ok(())
    }
}
