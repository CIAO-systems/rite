# Fake importer
This plugin provides an importer, that generates random records.
To use importer, include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="faker" name="rite_faker"/>
    </plugins>
</rite>
```
There are no named importers available, so `name` must be empty.
```xml
    <!-- ... -->
            <importer plugin="faker">
                <!-- Configuration goes here -->
            <importer>
    <!-- ... -->
```
## Importer
The importer generates a number of records with certain fields, according to the configuration.
The importer does not use any key/value configuration items, instead it reads the configuration from an XML file given in the attribute `xml`
### Example
```xml
<importer plugin="faker">
    <configuration xml="$RITE_CONFIG_PATH/faker-configuration.xml" />
</importer>
```
The variable `RITE_CONFIG_PATH` will be replaced with the path of the containing XML file.

### Configuration
The faker importer uses its own XML format for the configuration.
#### Example
```xml
<rite-random-import>
  <generator number="10">
     <field name="uuid-field-name" function="uuid" />
     <field name="milliseconds-field-name" function="milliseconds" />
     <field name="timezone-field-name" function="timezone" optional="true"/>
     <field name="string-field-name" function="string" />
     <field name="i32-field-name" function="i32" />
     <field name="f32-field-name" function="f32" />
     <!-- and so on and so forth -->
  </generator>
</rite-random-import>
```
#### rite-random-import
The root element of a faker importer configuration is `rite-random-import`
#### rite-random-import/generator
With the generator element the number of records and its fields are configured.
| **Attribute** | **Description** |
|---------------|-----------------|
| number | The number of records that should be generated

#### generator/field
The `generator` element can have multiple fields, that have the following attributes:
| **Attribute** | **Description** |
|---------------|-----------------|
| name | The nema of the generated field
| function | The name of the generating function. See [function list](#generation-function)
| optional | This boolean attribute controls, if a field is optional. If it is `true`, the importer will eventually add the field. So it can be part of the records sometimes. If it is `false` the field is added for every record.

##### Generation function
| **Function** | **Description** |
|---------------|-----------------|
| milliseconds | Creates a 'Value::I64` with the current time in milliseconds|
| timezone | Creates a random IANA timezone id |
| uuid | Creates a v4 UUID |

