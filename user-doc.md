# Overview

For easy usage, there is a shell script to run the container. See [the documentation for the script](rite.sh.md)

## RITE configuration format
A rite configuration is an [XML](https://developer.mozilla.org/en-US/docs/Web/XML/Guides/XML_introduction) file with the root element `rite`

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
