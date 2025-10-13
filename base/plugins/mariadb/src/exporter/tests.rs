mod manual {
    use model::{
        Initializable,
        export::Exporter,
        field::add_field,
        record::Record,
        xml::{
            common::{DatabaseConnection, Table},
            config::Configuration,
        },
    };
    use mysql::{Pool, prelude::Queryable};

    use crate::exporter::{MariaDBExporter, config::RiteMariaDBExport, insert, update};

    fn create_config() -> RiteMariaDBExport {
        RiteMariaDBExport {
            connection: DatabaseConnection {
                host: "localhost".into(),
                port: 3306,
                database: "app_db".into(),
                user: "root".into(),
                password: "rootpassword".into(),
            },
            table: Table {
                name: "test_users".into(),
                unique_fields: Some("id".into()),
                create: None,
            },
        }
    }

    fn create_test_record(id: i32, name: &str, age: i32) -> Record {
        let mut record = Record::new();
        let fields = record.fields_as_mut();
        add_field(fields, "id", model::value::Value::I32(id));
        add_field(fields, "name", model::value::Value::String(name.into()));
        add_field(fields, "age", model::value::Value::I32(age));
        record
    }

    #[test]
    #[ignore = "for manual testing"]
    fn test_insert_and_update() -> Result<(), Box<dyn std::error::Error>> {
        let pool = Pool::new("mysql://root:rootpassword@localhost:3306/app_db")?;
        let mut conn = pool.get_conn()?;

        // Create a table
        conn.query_drop(
            r#"
        CREATE TEMPORARY TABLE test_users (
            id INT PRIMARY KEY,
            name VARCHAR(100),
            age INT
        )
        "#,
        )?;

        let record = create_test_record(1, "Han Solo", 37);

        let config = create_config();
        let inserted = insert(&config, &mut conn, &record)?;
        assert_eq!(inserted, 1);

        let update_record = create_test_record(1, "Luke Skywalker", 28);
        let updated = update(&config, &mut conn, &update_record)?;
        assert_eq!(updated, 1);

        let row: Option<(i32, String, i32)> =
            conn.exec_first("SELECT id, name, age FROM test_users WHERE id = ?", (1,))?;

        assert!(row.is_some());
        let (id, name, age) = row.unwrap();
        assert_eq!(id, 1);
        assert_eq!(name, "Luke Skywalker");
        assert_eq!(age, 28);
        Ok(())
    }

    #[test]
    #[ignore = "for manual testing"]
    fn test_exporter() {
        let mut exporter = MariaDBExporter::new();
        let config = Configuration::with_xml("../../data/test/mariadb/manual_export.xml");
        let result = exporter.init(Some(config));
        println!("{:?}", result);
        assert!(result.is_ok());

        let record = create_test_record(1, "Anakin Skywalker", 73);
        let result = exporter.write(&record);
        println!("{:?}", result);
        assert!(result.is_ok());

        let update_record = create_test_record(1, "Leia Skywalker", 27);
        let result = exporter.write(&update_record);
        println!("{:?}", result);
        assert!(result.is_ok());

        let mut client = exporter.client.unwrap();
        let row: Option<(i32, String, i32)> = client
            .exec_first("SELECT id, name, age FROM test_users WHERE id = ?", (1,))
            .unwrap();
        println!("{:?}", row);

        assert!(row.is_some());
        let (id, name, age) = row.unwrap();
        assert_eq!(id, 1);
        assert_eq!(name, "Leia Skywalker");
        assert_eq!(age, 27);
    }
}
