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
Currently, the CSV file has to have a header row with the names of the fields as first line and must use `,` as delimiter

### Configuration
| Key | Description |
| --- | --- |
| filename | The path to the .csv file

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
Currently, the CSV file has a header row with the names of the fields as first line and uses `,` as delimiter

### Configuration
| Key | Description |
| --- | --- |
| filename | The path to the .csv file
| overwrite | A boolean value, that indicates, if the file should be overwritten
