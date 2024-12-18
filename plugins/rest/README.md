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



