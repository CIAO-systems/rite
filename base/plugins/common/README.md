# Common functionality for [RITE](../../../README.md)
This plugin contains some common transformers and exporters. To use them, 
include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="common" name="rite_common"/>
    </plugins>
</rite>
```

## Importer
[Documentation of the common importers](importer.md)
## Transformer
[Documentation of the common transformers](transformer.md)
## Exporter
[Documentation of the common exporters](exporter.md)
