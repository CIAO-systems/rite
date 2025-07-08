# CIAO RITE plugin
A plugin for importing data from CIAO or export data to CIAO 

To use any of the [importers](doc/importers.md) or [exporters](doc/exporters.md), include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="ciao" name="rite_ciao"/>
    </plugins>
</rite>
```
## Common Configuration
All plugins have the following configuration in common:
```xml
<configuration>
    <config key="url" value="$CIAO_URL" />
    <config key="api-key" value="$CIAO_API_KEY" />
</configuration>
```
| Key | Description |
| --- | --- |
| `url` | The URL of the CIAO backend gRPC API Server |
| `api-key`| The API key for authentication with the gRPC services |


## Importers
[CIAO importer documentation](doc/importers.md)

## Exporter
[CIAO exporter documentation](doc/exporters.md)
