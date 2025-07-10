# ATOSS Time Control RITE plugin
A plugin for importing data from ATOSS Time Control

To use one of the importers, include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="atc" name="rite_atc"/>
    </plugins>
</rite>
```

Currently, the plugin only supports basic authentication.

## Common Configuration
All importers have the following configuration in common:
```xml
<configuration>
    <config key="url" value="$ATC_URL" />
    <config key="auth-token" value="<client-identfier>" />
    <config key="user" value="$ATC_USER" />
    <config key="password" value="$ATC_PASSWORD" />
</configuration>
```
| Key | Description |
| --- | --- |
| `url` | The URL of the ATOSS Time Control gRPC API Server, for example `https://example-grpc.com:443` |
| `auth-token`| A client identifier, can be any value |
| `user`| A ATOSS Time Control employee ID |
| `password`| The password of the ATOSS Time Control employee |


## Importers
* [Absences importer](doc/importer/absences.md)
* [Generic Dataset importer](doc/importer/dataset.md)
* [Clock records importer](doc/importer/clock_records.md)
