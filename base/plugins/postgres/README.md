# Postgres import and export
This plugin provides an importer and an exporter for the Postgres database
To use importer or the exporter, include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="postgres" name="rite_postgres"/>
    </plugins>
</rite>
```
There are no named importers or exporters available, so `name` must be empty.
```xml
    <!-- ... -->
            <importer plugin="postgres">
                <!-- Configuration goes here -->
            <importer>
            <exporters>
                <exporter plugin="postgres">
                    <!-- Configuration goes here -->
                <exporter>
            <exporters>
    <!-- ... -->
```
## Importer
The importer takes an SQL SELECT statement to read records and converts them to 
the `model::Record` that is then passed to the transformers and exporters.
The importer does not use any key/value configuration itmes, instead it reads the configuration from an XML file given in the attribute `xml`
### Example
```xml
<importer plugin="postgres">
    <configuration xml="$RITE_CONFIG_PATH/postgres-import-config.xml" />
</importer>
```
The variable `RITE_CONFIG_PATH` will be replaced with the path of the containing XML file.

### Configuration
The Postgres importer uses its own XML format for the configuration.
#### Example
```xml
<rite-postgres-import>
    <connection 
        host="localhost"
        port="5432"
        database="postgres"
        user="postgres"
        password="${POSTGRES_PASSWROD:6d598907-a775-4383-ab6f-de525c5ac0bf}"
    />
    <sql>select * from customers</sql>
</rite-postgres-import>
```
#### rite-postgres-import
The root element of a postgres importer configuration is `rite-postgres-import`
#### rite-postgres-import/connection
With the connection element the database connecton information is configured.
| **Attribute** | **Description** |
|---------------|-----------------|
| host | Host or IP address of the Postgres server
| port| Postgres port
| database | name of the database
| user| Username
| password | Password
For better security, it is recommended to use an environment variable for the `password` (see example above)
#### rite-postgres-import/sql
This element contains the SQL statement, that will be executed to get a result set, that will be converted to records for further processing.

## Exporter
The exporter writes the transformed record to the configured table. The exporter does not use any key/value configuration itmes, instead it reads the configuration from an XML file given in the attribute `xml`

### Example
```xml
<exporter plugin="rite_postgres">
    <configuration xml="$RITE_CONFIG_PATH/postgres-export-config.xml" />
</exporter>
```                

### Configuration
The Postgres exporter uses its own XML format for the configuration.
#### Example
```xml
<rite-postgres-export>
    <connection 
        host="localhost"
        port="5432"
        database="postgres"
        user="postgres"
        password="${POSTGRES_PASSWROD:6d598907-a775-4383-ab6f-de525c5ac0bf}"
    />
    <table name="backup_customer" uniqueFields="id">
        <create>
        <![CDATA[
CREATE TABLE IF NOT EXISTS backup_customer (
	id serial4 NOT NULL,
	"name" varchar NOT NULL,
	CONSTRAINT backup_customers_pkey PRIMARY KEY (id)
);        
        ]]>
        </create>
    </table>
</rite-postgres-export>
```
#### rite-postgres-export
The root element of a postgres exporter configuration is `rite-postgres-export`
#### rite-postgres-export/connection
This is the same as the [connection configuration of the import](#rite-postgres-importconnection)
#### rite-postgres-export/table
| **Attribute** | **Description** |
|---------------|-----------------|
| name | The name of the target table
| uniqueFields | The fields of the primary index (as comma separated list). This is used, after a key violation on INSERT to create an UPDATE statement filter.
#### rite-postgres-export/table/create
The `create` element contains the Postgres SQL statement to create the table. This element is optional. If it does not exist, the table will not be automatically created, but it is assumed, that the table already exists.





