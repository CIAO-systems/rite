# CIAO RITE plugin
A plugin for importing data from CIAO or export data to CIAO 

To use one of the importers, include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="ciao" name="rite_ciao"/>
    </plugins>
</rite>
```
## Common Configuration
All plugins have the following configuration in common:
```xml
<configuration>
    <config key="url" value="$CIAO_URL" />
    <config key="api-key" value="$CIAO_API_KEY" />
</configuration>
```
| Key | Description |
| --- | --- |
| `url` | The URL of the CIAO backend gRPC API Server |
| `api-key`| The API key for authentication with the gRPC services |


## Importers
### Accounts
This importer reads the accounts. 
To use it, define a process and add the importer with the name `accounts` to your project:
```xml
<importer plugin="ciao" name="accounts">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
        <!-- Filter parameters -->
    </configuration>
</importer>
```
Currently, this importer retrieves all accounts
#### Fields
| Field | Type | Description |
| --- | --- | --- |
| `id` | String | Unique id of the account |
| `email` | String | E-Mail address of the account |
| `name.first` | String | First name of the account |
| `name.middle` | String | Middle name  of the account |
| `name.last` | String | Last name of the account |
| `address.city` | String | City of the account |
| `address.postal_code` | String | Postal code of the account |
| `address.address_line_1` | String | Address line one of the account |
| `address.address_line_2` | String | Address line two of the account |
| `address.region_code` | String | Region code of the account |
| `address.state` | String | State of the account |
| `avatar.id` | String | Avatar id of the account |
| `avatar.updatedAt.timeUtc` | String | Time (in UTC unix time millis) of the last change for the avatar of the account |
| `avatar.updatedAt.timeZone` | String | Time zone (IANA time zone id) of the last change for the avatar of the account |



### Devices
*TBD*

### Projects
This importer reads the projects. 
To use it, define a process and add the importer with the name `projects` to your project:
```xml
<importer plugin="ciao" name="projects">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
    </configuration>
</importer>
```

Currently, this importer retrieves all projects

### Time types
This importer reads the time types. 
To use it, define a process and add the importer with the name `time_types` to your project:
```xml
<importer plugin="ciao" name="time_types">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
        <!-- Filter parameters -->
        <config key="filter.absence" value="true" />
        <config key="filter.bookable" value="true" />
    </configuration>
</importer>
```
This importer supports the additional filter parameters:
| Key | Description | Type | Default |
| --- | --- | --- | --- |
| `filter.absence` | Only return time types, that match the value given here for the time type option `absence` | bool | \<None> |
| `filter.bookable`| Only return time types, that match the value given here for the time type option `bookable` | bool | \<None> |

### Clock entries
This importer reads the clock entries.
To use it, define a process and add the importer with the name `clock_entries` to your project:
```xml
<importer plugin="ciao" name="clock_entries">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
            <!-- Filter parameter 
            <config key="filter.timeRange.startTime" value="2024-01-01T00:00:00Z" />
            <config key="filter.timeRange.endTime" value="2024-01-02T00:00:00Z" />
            <config key="filter.userId" value="<user-id>" />
            <config key="filter.creatorId" value="<creator-id>" />
            <config key="filter.timeTypeId" value="<time-type-id" />
            -->
    </configuration>
</importer>
```
This importer supports the additional filter parameters:
| Key | Description | Type | Default |
| --- | --- | --- | --- |
| filter.timeRange.startTime | Only return clock entries after this date/time | ISO 8601 | \<None> |
| filter.timeRange.endTime | Only return clock entries before this date/time | ISO 8601 | \<None> |
| filter.userId | Only return clock entries for this user id | UUID | \<None> |
| filter.creatorId | Only return clock entries created by this user | UUID | \<None> |
| filter.timeTypeId | Only return clock entries for this time type | UUID | \<None> |


