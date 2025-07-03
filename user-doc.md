# Overview

For easy usage, there is a shell script to run the container. See [the documentation for the script](rite.sh.md)

## RITE configuration format
A rite configuration is an [XML](https://developer.mozilla.org/en-US/docs/Web/XML/Guides/XML_introduction) file with the root element `rite`

A rit configuration must have the elements `plugins` and `processes`. A `process` element within the `processes` must contain one `importer` element and at least one `exporter` element withing the mandatory `exporters` element. The element `transformers` and its `transformer` elements are optional.

A minimal rite configuration has those elements:
```xml
<rite>
    <plugins>
        <plugin ... />
    </plugins>
    <processes>
        <process>
            <importer ...>
            </importer>
            <!-- Optional transformers
            <transformers>
                <transformer ...>
                </transformer>
            </transformers>
            -->
            <exporters>
                <exporter ...>
                </exporter>
            </exporters>
        </process>
    </processes>
</rite>
```

In all of the configuration files, environment variables can be used. When using the rite container image, make sure to use the `--env` parameter for the `rite.sh` script to load environment variables from the given file. 

There are also some predefined variables, that can be used in any configuration XML:

### Predefined RITE variables
| Variable | Description |
| :--- | :--- |
| `RITE_CONFIG_PATH` | The variable will be replaced with the directory, where the XML file is located

#### Example
The postgres importer has a separate configuration XML that can be loaded from the same directory of the main configuration:
```xml
    <importer plugin="rite_postgres">
        <configuration xml="$RITE_CONFIG_PATH/postgres-import-config.xml" />
    </importer>
```

### XML configuration elements
#### Root elelemt
The root element of the rite configuration is `rite`. It has no attributes.
##### Example
```xml
<rite>
</rite>
```

#### Plugins element
Every rite import/export needs plugins to be functional. Which plugins can be used, is defined in the `plugins` element. It contains one or many `plugin` elements. The `plugins` element has no attributes.

##### Example
```xml
<rite>
    <plugins>
    </plugins>
</rite>
```

##### Plugin element
A plugin element describes where `rite` finds a plugin that offers importers, transfomers or exporters.

###### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common" />
    </plugins>
</rite>
```
###### Attributes
| Attribute | Description | Required |
| :--- | :--- | :--- |
| `id` | The id by which this plugin is referenced in any `importer`, `transformer` or `exporter` | yes |
| `path` | The path to the dynamic library, where rite can find the library file. If the dynamic library is in the library search path (`LD_LIBRARY_PATH` on Linux), the path attribute can be ommited | no |
| `name` | The name of the dynamic library file | yes |

#### Processes element
##### Process element
###### Importer element
###### Transformers element
###### Transformer element
###### Exporters element
###### Exporter element

#### Configuration element
All processing elements (`importer`, `transformer` and `exporter`) support the `configuration` element.
##### Example
##### Attributes
##### Config element


