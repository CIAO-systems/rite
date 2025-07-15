# Clock records
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
## Filter
| Key | Description |
| --- | --- | 
| `filter.employee` | When given, the result is filtered for this employee-id. Only clock records of this emplloyee will be returned  |
| `filter.period` | When given, the result will be restricted to the interval from `start-date` to `end-date`. The values are separated by `:`. The format for a date is `Year-Month-Day`. If any of the values is empty, it is an open interval. <br> For example, if the `filter.period` is `:2025-12-31`, then all records before the 2025-12-31 are listed. If it is `2025-12-31:` then all records after 2025-12-31 are listed. All dates are exclusive.
| `filter.fields` | This comma separated string determines, which fields should be included in each record. See the clock record table documentation of ATOSS Time Control for details about the fields |

## Fields
The fields from the ATOSS Time Control table `Clockin`.

