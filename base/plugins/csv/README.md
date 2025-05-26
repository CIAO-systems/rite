# CSV plugin
To use one of the importers or exporters, include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="csv" name="rite_csv"/>
    </plugins>
</rite>
```
## Common configuration
For the exporter and the importer, the following configuration is available:

| Key | Type | Description | Default
| --- | --- | --- | --- |
| filename | String | The path to the .csv file | |
| delimiter | Char | The field delimiter to be used. | `,`

## Importer
The CSV importer reads from a CSV file

To use it, define a process and add the importer (without name) to your project:
```xml
<importer plugin="csv">
    <configuration>
        <config key="filename" value="path/to/csv/file" />
    </configuration>
</importer>
```

## Exporter
The CSV exporter writes a CSV file

To use it, define a process and add the importer (without name) to your project:
```xml
<exporter plugin="csv">
    <configuration>
        <config key="filename" value="path/to/csv/file" />
    </configuration>
</exporter>
```
Currently, the CSV file has a header row with the names of the fields as first line 

### Configuration
| Key | Type | Description | Default
| --- | --- | --- | --- |
| overwrite | bool | A boolean value, that indicates, if the file should be overwritten | false
