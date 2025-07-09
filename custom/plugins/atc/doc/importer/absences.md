# Absences
This importer reads the absences of an ATOSS Time Control installation.
To use it, define a process and add the importer with the name `absences` to your project:
```xml
<importer plugin="atc" name="absences">
    <configuration>
        <config key="url" value="$ATC_URL" />
        <config key="auth-token" value="rite" />
        <config key="user" value="$ATC_USER" />
        <config key="password" value="$ATC_PASSWORD" />
        <!-- Filters -->
        <config key="filter.employees" value="<commalist-of-atc-employee-ids>" />
        <config key="filter.period" value="<start-date>:<end-date>" />
        <config key="filter.accounts" value="<commalist-of-atc-account-ids>" />
    </configuration>
</importer>
```
## Filter
| Key | Description |
| --- | --- | 
| `filter.employees` | When given, the result is filtered for this list of employee-ids. Only absences of those employees will be returned  |
| `filter.period` | When given, the result will be restricted to the interval from `start-date` to `end-date`. The values are separated by `:`. The format for a date is `Year-Month-Day`. If the start date is missing, the current date will be used as start date. If the end date is missing, the period will end one year after the start date.
| `filter.accounts` | When given, the result is filtered for this list of account-ids. Only absences for those accounts will be returned |

## Fields
The fields from the ATOSS Time Control gRPC call `AbsenceService.getSingleDayAbsences`
| Field | Type |
| :--- | :---: |
| `accountId` | i32 |
| `application` | String |
| `description` | String |
| `displayColor` | i32 |
| `displayToken` | String |
| `employeeId` | String |
| `endDate` | ISO 8601 string |
| `planVersion` | i32 |
| `remark` | String |
| `startDate` | ISO 8601 string |
| `state` | i32 |
| `substitute` | String |
| `textColor` | i32 |
| `weightEnd` | f64 |
| `weightStart` | f64 |

