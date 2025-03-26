# Personio import
This plugin provides importers for the Personio API
To use an importer, include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="personio" name="rite_personio"/>
    </plugins>
</rite>
```
## Common configuration
To access the Personio API, client id and client secret have to be provided.
The importer will retrieve an access token using the Persion authentication API.
| Key | Description |
| --- | --- |
| client_id | The Personio client id |
| client_secret | The secret for the `client_id` |
| X-Personio-Partner-ID | See [Personio documentation](https://developer.personio.de/v1.0/reference/include-our-headers-in-your-requests) |
| X-Personio-App-ID | See [Personio documentation](https://developer.personio.de/v1.0/reference/include-our-headers-in-your-requests) |

## Importers
### Employees
The `employees` importer reads the personio company employees using the Personio API v1 (useing the project [personio-rs](https://github.com/CIAO-systems/personio-rs)) 
```xml
    <!-- ... -->
            <importer plugin="personio" name="employees">
                <!-- Configuration goes here -->
            <importer>
    <!-- ... -->
```
#### Configuration
| Key | Description | Default |
| --- | --- | --- |
| flags.salary | Include the salary fields  | false
| options.limit | Page size for pagination  | 10
| filter.email | Only return the data for the employee with this email address | -
| filter.updated_since | Only return records that have been updated since the given timestamp. Value is an ISO 8601 date/time string. For example: `2025-01-01T00:00:00` | -
| filter.attributes | A comma separated list of fields to include in the result. For custom fields, the `key` is used although in the output it might be named with the `universal_id`. For example, the filter field `dynamic_12545739` will return the field `date_of_birth` | -

#### Results
The resulting records will contain the fields as described in the Personio API V1 
documentation for the [company employees](https://developer.personio.de/v1.0/reference/get_company-employees)

#### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common"/>
        <plugin id="personio" name="rite_personio"/>
    </plugins>
    <processes>
        <process id="Personio employees">
            <importer plugin="personio" name="employees">
                <configuration>
                    <config key="client_id" value="$PERSONIO_CLIENT_ID" />
                    <config key="client_secret" value="$PERSONIO_CLIENT_SECRET" />

                    <config key="filter.attributes" value="preferred_name,dynamic_12545745,dynamic_12545766,dynamic_12545739" />

                </configuration>
            </importer>
            <exporters>
                <exporter plugin="common" name="console">
                    <configuration>
                        <config key="prefix" value="Employee {" />
                        <config key="postfix" value="}" />
                        <config key="separator" value="&#xA;" />
                        <config key="field-prefix" value=" " />
                    </configuration>
                </exporter>
            </exporters>
        </process>
    </processes>
</rite>
```

### Projects
The `projects` importer reads the personio company projects using the Personio API v1 (useing the project [personio-rs](https://github.com/CIAO-systems/personio-rs)) 
```xml
    <!-- ... -->
            <importer plugin="personio" name="employees">
                <!-- Configuration goes here -->
            <importer>
    <!-- ... -->
```
#### Configuration
Currently there is no projects specific configuration available

#### Results
The resulting records will contain the fields as described in the Personio API V1 
documentation for the [company projects](https://developer.personio.de/v1.0/reference/get_company-attendances-projects)

#### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common"/>
        <plugin id="personio" name="rite_personio"/>
    </plugins>
    <processes>
        <process id="Personio projects">
            <importer plugin="personio" name="projects">
                <configuration>
                    <config key="client_id" value="$PERSONIO_CLIENT_ID" />
                    <config key="client_secret" value="$PERSONIO_CLIENT_SECRET" />

                    <config key="X-Personio-Partner-ID" value="CIAO Systems GmbH" />
                    <config key="X-Personio-App-ID" value="rite" />

                </configuration>
            </importer>
            <exporters>
                <exporter plugin="common" name="console">
                    <configuration>
                        <config key="prefix" value="Project {" />
                        <config key="postfix" value="}" />
                        <config key="separator" value="&#xA;" />
                        <config key="field-prefix" value=" " />
                    </configuration>
                </exporter>
            </exporters>
        </process>
    </processes>
</rite>
```