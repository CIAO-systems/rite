# REST import
This plugin provides an importer for REST services. Currently only service without authentication are supported.
To use importer, include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="rest" name="rite_rest"/>
    </plugins>
</rite>
```
There are no named importers available, so `name` must be empty.
```xml
    <!-- ... -->
            <importer plugin="rest">
                <!-- Configuration goes here -->
            <importer>
    <!-- ... -->
```
## Importer
The REST importer calls the configured REST services with `GET` and creates records from the result.
### Configuration
The importer uses the following keys from the configuration:
| **Key** | **Value** |
|---------------|-----------------|
| url | URL of the REST service
| records_field | When given, the actual records can be found in the JSON array with the name of the value given here
| fields_path | When given, fields for the imported record will be taken from the JSON object from this path.<br>For example, if the actual record is stored in an attribute called `properties`, like this ```[{ "properties": { "field1": "value1" }}] ```, the object can be accessed by setting `fields_path` to `properties`. The path can be separated with dot syntax. For example: `properties.field1`, if the record is stored in `field1` of `properties`
| auth.basic | For basic authentication, the user and password can be provided (separated with `:`). For example: `<config key="auth.basic" value="obiwan:the force is strong in this passw0rd" />` |
| auth.bearer | The token for bearer token authentication. For example: `<config key="auth.bearer" value="a-token-that-nobody-will-ever-guess" />`|
| auth.api-key | An API key. The name of the header value (usually `x-api-key`) must be provided and the value, separated by `:`. For example: `<config key="auth.api-key" value="x-api-key:$ENV_SECRET_API_KEY" />` |

#### Authentication
Authentication methods can be combined. This is especially important if a service needs an API key and a user authentication. Basic authentication and bearer authentication cannot be combined. If both are configured, bearer token authentication will take precedence.


### Example
```xml
<configuration>
    <config key="url" value="https://swapi.dev/api/films" />
    <config key="records_field" value="results" />
</configuration>
```
In the above example, `rite` will retrieve all Star Wars films from the Star Wars API and take the structures in the field `results` as records of the import.
In case the Star Wars API is not available on https://swapi.dev, you can try https://swapi.info or 
https://swapi.online



