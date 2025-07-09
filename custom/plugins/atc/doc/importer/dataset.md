# Dataset
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
## Filter
| Key | Description |
| --- | --- | 
| `filter.table` | The name of the ATC table to read  |
| `filter.fields` | Thos comma separated strin determines, which fields should be included in each record. Some fields will be returned by ATC though, no matter if they are given in the list of fields or not (primary key fields, mostly) |

## Fields
The fields returned by this importer depend on the table. For a detaild description of the fields of any table, please read the documentation of ATOSS Time Control
