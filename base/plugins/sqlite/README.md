# SQLite import and export
This plugin provides an importer and an exporter for a SQLite database
To use importer or the exporter, include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="sqlite" name="rite_sqlite"/>
    </plugins>
</rite>
```
There are no named importers or exporters available, so `name` must be empty.
```xml
    <!-- ... -->
            <importer plugin="sqlite">
                <!-- Configuration goes here -->
            <importer>
            <exporters>
                <exporter plugin="sqlite">
                    <!-- Configuration goes here -->
                <exporter>
            <exporters>
    <!-- ... -->
```
## Importer
The importer takes an SQL SELECT statement to read records and converts them to 
the `model::Record` that is then passed to the transformers and exporters.
The importer does not use any key/value configuration items, instead it reads the configuration from an XML file given in the attribute `xml`
### Example
```xml
<importer plugin="sqlite">
    <configuration xml="$RITE_CONFIG_PATH/sqlite-import-config.xml" />
</importer>
```
The variable `RITE_CONFIG_PATH` will be replaced with the path of the containing XML file.

### Configuration
The SQLite importer uses its own XML format for the configuration.
#### Example
```xml
<rite-sqlite-import>
    <filename>$RITE_CONFIG_PATH/customers.db</filename>
    <sql>select * from customers</sql>
</rite-sqlite-import>
```
#### rite-sqlite-import
The root element of a sqlite importer configuration is `rite-sqlite-import`
#### filename
This element contains the path to the SQLite database file
#### sql
This element contains the SQL statement, that will be executed to get a result set, that will be converted to records for further processing.

## Exporter
The exporter writes the transformed record to the configured table. The exporter does not use any key/value configuration items, instead it reads the configuration from an XML file given in the attribute `xml`

### Example
```xml
<exporter plugin="rite_sqlite">
    <configuration xml="$RITE_CONFIG_PATH/sqlite-export-config.xml" />
</exporter>
```                

### Configuration
The sqlite exporter uses its own XML format for the configuration.
#### Example
```xml
<rite-sqlite-export>
    <filename>$RITE_CONFIG_PATH/customers.db</filename>
    <table name="backup_customer" uniqueFields="id">
        <create>
        <![CDATA[
CREATE TABLE backup_customers (
	id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
	name TEXT(255)
);
        ]]>
        </create>
    </table>
</rite-sqlite-export>
```
#### rite-sqlite-export
The root element of a sqlite exporter configuration is `rite-sqlite-export`
#### filename
This element contains the path to the SQLite database file
#### table
| **Attribute** | **Description** |
|---------------|-----------------|
| name | The name of the target table
| uniqueFields | The fields of the primary index (as comma separated list). This is used, after a key violation on INSERT to create an UPDATE statement filter.
#### table/create
The `create` element contains the SQL statement to create the table. This element is optional. If it does not exist, the table will not be automatically created, but it is assumed, that the table already exists.





