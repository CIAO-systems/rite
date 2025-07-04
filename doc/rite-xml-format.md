# RITE configuration format
A rite configuration is an [XML](https://developer.mozilla.org/en-US/docs/Web/XML/Guides/XML_introduction) file with the root element `rite`

A rite configuration must have the elements `plugins` and `processes`. A `process` element within the `processes` must contain one `importer` element and at least one `exporter` element withing the mandatory `exporters` element. The element `transformers` and its `transformer` elements are optional.

A minimal rite configuration looks as follows:
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

## Predefined RITE variables
| Variable | Description |
| :--- | :--- |
| `RITE_CONFIG_PATH` | The variable will be replaced with the directory, where the XML file is located

### Example
The [postgres importer](base/plugins/postgres/README.md) has a separate configuration XML that can be loaded from the same directory of the main configuration:
```xml
    <importer plugin="rite_postgres">
        <configuration xml="$RITE_CONFIG_PATH/postgres-import-config.xml" />
    </importer>
```

## Root elelemt
The root element of the rite configuration is `rite`. It has no attributes.
### Example
```xml
<rite>
</rite>
```

## Plugins element
Every rite import/export needs plugins to be functional. Which plugins can be used, is defined in the `plugins` element. It contains one or many `plugin` elements. The `plugins` element has no attributes.

### Example
```xml
<rite>
    <plugins>
    </plugins>
</rite>
```

### Plugin element
A plugin element describes where `rite` finds a plugin that offers importers, transfomers or exporters.

#### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common" />
    </plugins>
</rite>
```
#### Attributes
| Attribute | Description | Required |
| :--- | :--- | :--- |
| `id` | The id by which this plugin is referenced in any `importer`, `transformer` or `exporter` | yes |
| `path` | The path to the dynamic library, where rite can find the library file. If the dynamic library is in the library search path (`LD_LIBRARY_PATH` on Linux), the path attribute can be ommited | no |
| `name` | The name of the dynamic library file | yes |

## Processes element
In a rite configuration there can be multiple processes. Each process must have one `importer` that reads data from a datasource. There must also be at least one `exporter`, that writes data to a data sink. Every process can have one or more `transformer` elements.

### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common" />
    </plugins>
    <processes>
    </processes>
</rite>
```

### Process element
A `process` element describes the actual import/transform/export.

| Attribute | Description | Required |
| :--- | :--- | :--- |
| `id` | The id by which this process is identified | yes |

#### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common" />
    </plugins>
    <processes>
        <process id="print.env" >
        </process>
    </processes>
</rite>
```

#### Importer element
The mandatory `importer` element describes the datasource. 

| Attribute | Description | Required |
| :--- | :--- | :--- |
| `plugin` | The id of the plugin of the importer. See [plugins](#plugins-element) | yes |
| `name` | The optional name of the importer from the plugin, if the plugin offers named importers. | no |

An `importer` element can have an optional [configuration](#configuration-element) element.

##### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common" />
    </plugins>
    <processes>
        <process id="print.env" >
            <importer plugin="common" name="env" />
        </process>
    </processes>
</rite>
```

#### Transformers element
The list of optional transformers describes all the `transformer` elements, that can be used, to change read records before they are transferred to the exporters.

##### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common" />
    </plugins>
    <processes>
        <process id="print.env" >
            <importer plugin="common" name="env" />

            <transformers>
            </transformers>
        </process>
    </processes>
</rite>
```

##### Transformer element
A `transformer` element describes a transforming function for the imported records.

| Attribute | Description | Required |
| :--- | :--- | :--- |
| `plugin` | The id of the plugin of the transformer. See [plugins](#plugins-element) | yes |
| `name` | The optional name of the transformer from the plugin, if the plugin offers named transformers. | no |

A `transformer` element can have an optional [configuration](#configuration-element) element.

###### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common" />
    </plugins>
    <processes>
        <process id="print.env" >
            <importer plugin="common" name="env" />

            <transformers>
                <transformer plugin="common">
                    <configuration>
                        <config key="add_field" value="id:uuid" />
                    </configuration>
                </transformer>
            </transformers>
        </process>
    </processes>
</rite>
```

##### Exporters element
The list of exporters describes all the `exporter` elements, that can be used, to write records.
There must be at least one `exporter` element.

###### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common" />
    </plugins>
    <processes>
        <process id="print.env" >
            <importer plugin="common" name="env" />

            <transformers>
                <transformer plugin="common">
                    <configuration>
                        <config key="add_field" value="id:uuid" />
                    </configuration>
                </transformer>
            </transformers>

            <exporters>
            </exporters>
        </process>
    </processes>
</rite>
```

###### Exporter element
An `exporter` element describes a writing function for the imported records.

| Attribute | Description | Required |
| :--- | :--- | :--- |
| `plugin` | The id of the plugin of the exporter. See [plugins](#plugins-element) | yes |
| `name` | The optional name of the exporter from the plugin, if the plugin offers named exporterss. | no |

An `exporter` element can have an optional [configuration](#configuration-element) element.

###### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common" />
    </plugins>
    <processes>
        <process id="print.env" >
            <importer plugin="common" name="env" />

            <transformers>
                <transformer plugin="common">
                    <configuration>
                        <config key="add_field" value="id:uuid" />
                    </configuration>
                </transformer>
            </transformers>

            <exporters>
                <exporter plugin="common" name="console" />
            </exporters>
        </process>
    </processes>
</rite>
```

## Configuration element
All processing elements (`importer`, `transformer` and `exporter`) support the `configuration` element.
### Example
```xml
    <configuration xml="$RITE_CONFIG_PATH/extra-configuration.xml">
        <config key="config-key" value="config-value" />
    </configuration>
```

| Attribute | Description | Required |
| :--- | :--- | :--- |
| `xml` | An optional value to point to an extra XML configuration file. If the attribute is needed depends on the actual plugin | no |

### Config element
A config element is a key/value pair.

| Attribute | Description | Required |
| :--- | :--- | :--- |
| `key` | The name of the configuration variable. See the plugin documentation, what config keys are supported | yes |
| `value` | The value of the config item | yes |


