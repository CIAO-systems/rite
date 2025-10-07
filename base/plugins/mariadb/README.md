# MariaDB/MySQL import and export
This plugin provides an importer and an exporter for the MariaDB/MySQL database
To use importer or the exporter, include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="mariadb" name="rite_mariadb"/>
    </plugins>
</rite>
```
There are no named importers or exporters available, so `name` must be empty.
```xml
    <!-- ... -->
            <importer plugin="mariadb">
                <!-- Configuration goes here -->
            <importer>
            <exporters>
                <exporter plugin="mariadb">
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
<importer plugin="mariadb">
    <configuration xml="$RITE_CONFIG_PATH/mariadb-import-config.xml" />
</importer>
```
The variable `RITE_CONFIG_PATH` will be replaced with the path of the containing XML file.

### Configuration
The MariaDB importer uses its own XML format for the configuration.
#### Example
```xml
<rite-mariadb-import>
    <connection 
        host="localhost"
        port="3306"
        database="mariadb"
        user="mariadb"
        password="${MARIADB_PASSWROD:6d598907-a775-4383-ab6f-de525c5ac0bf}"
    />
    <sql>select * from customers</sql>
</rite-mariadb-import>
```
#### rite-mariadb-import
The root element of a mariadb importer configuration is `rite-mariadb-import`
#### rite-mariadb-import/connection
With the connection element the database connecton information is configured.
| **Attribute** | **Description** |
|---------------|-----------------|
| host | Host or IP address of the mariadb server
| port| mariadb port
| database | name of the database
| user| Username
| password | Password
For better security, it is recommended to use an environment variable for the `password` (see example above)
#### rite-mariadb-import/sql
This element contains the SQL statement, that will be executed to get a result set, that will be converted to records for further processing.

## Exporter
The exporter writes the transformed record to the configured table. The exporter does not use any key/value configuration itmes, instead it reads the configuration from an XML file given in the attribute `xml`

### Example
```xml
<exporter plugin="rite_mariadb">
    <configuration xml="$RITE_CONFIG_PATH/mariadb-export-config.xml" />
</exporter>
```                

### Configuration
The mariadb exporter uses its own XML format for the configuration.
#### Example
```xml
<rite-mariadb-export>
    <connection 
        host="localhost"
        port="3306"
        database="mariadb"
        user="mariadb"
        password="${mariadb_PASSWROD:6d598907-a775-4383-ab6f-de525c5ac0bf}"
    />
    <table name="backup_customer" uniqueFields="id">
        <create>
        <![CDATA[
CREATE TABLE backup_customer (
  id INT NOT NULL UNIQUE,
  name VARCHAR(255) NOT NULL
);        
        ]]>
        </create>
    </table>
</rite-mariadb-export>
```
#### rite-mariadb-export
The root element of a mariadb exporter configuration is `rite-mariadb-export`
#### rite-mariadb-export/connection
This is the same as the [connection configuration of the import](#rite-mariadb-importconnection)
#### rite-mariadb-export/table
| **Attribute** | **Description** |
|---------------|-----------------|
| name | The name of the target table
| uniqueFields | The fields of the primary index (as comma separated list). This is used, after a key violation on INSERT to create an UPDATE statement filter.
#### rite-mariadb-export/table/create
The `create` element contains the MariaDB SQL statement to create the table. This element is optional. If it does not exist, the table will not be automatically created, but it is assumed, that the table already exists.





