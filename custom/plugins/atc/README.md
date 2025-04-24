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
### Dataset
This importer reads data from any table in ATC. 
To use it, define a process and add the importer with the name `dataset` to your project:
```xml
<importer plugin="atc" name="dataset">
    <configuration>
        <config key="url" value="$ATC_URL" />
        <config key="auth-token" value="<client-identfier>" />
        <config key="user" value="$ATC_USER" />
        <config key="password" value="$ATC_PASSWORD" />
        <!-- Filters -->
        <config key="filter.table" value="Project" />
        <config key="filter.fields" value="name,value,actualstart" />
    </configuration>
</importer>
```

Currently, this importer retrieves all records of the given `table`
#### Filter
| Key | Description |
| --- | --- | 
| `filter.table` | The name of the ATC table to read  |
| `filter.fields` | Thos comma separated strin determines, which fields should be included in each record. Some fields will be returned by ATC though, no matter if they are given in the list of fields or not (primary key fields, mostly) |

#### Fields
The fields returned by this importer depend on the table. For a detaild description of the fields of any table, please read the documentation of ATOSS Time Control

### Clock records
This importer reads the clock records of an ATOSS Time Control installation.
To use it, define a process and add the importer with the name `clock_records` to your project:
```xml
<importer plugin="atc" name="clock_records">
    <configuration>
        <config key="url" value="$ATC_URL" />
        <config key="auth-token" value="rite" />
        <config key="user" value="$ATC_USER" />
        <config key="password" value="$ATC_PASSWORD" />
        <!-- Filters -->
        <config key="filter.employee" value="<atc-employee-id>" />
        <config key="filter.period" value="<start-date>:<end-date>" />
        <config key="filter.fields" value="<field-list>" />
    </configuration>
</importer>
```
#### Filter
| Key | Description |
| --- | --- | 
| `filter.employee` | When given, the result is filtered for this employee-id. Only clock records of this emplloyee will be returned  |
| `filter.period` | When given, the result will be restricted to the interval from `start-date` to `end-date`. The values are separated by `:`. The format for a date is `Year-Month-Day`. If any of the values is empty, it is an open interval. <br> For example, if the `filter.period` is `:2025-12-31`, then all records before the 2025-12-31 are listed. If it is `2025-12-31:` then all records after 2025-12-31 are listed. All dates are exclusive.
| `filter.fields` | This comma separated string determines, which fields should be included in each record. See the clock record table documentation of ATOSS Time Control for details about the fields |

#### Fields
The fields from the ATOSS Time Control table `Clockin`.

