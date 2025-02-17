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
### Cost centers
This importer reads the cost centers. 
To use it, define a process and add the importer with the name `cost_centers` to your project:
```xml
<importer plugin="ciao" name="cost_centers">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
    </configuration>
</importer>
```

Currently, this importer retrieves all cost centers
#### Fields
| Field | Type | Description |
| --- | --- | --- |
| `id` | String | Unique id of the cost center |
| `name` | String | Descriptive name of the cost center |

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
This importer reads the devices. 
To use it, define a process and add the importer with the name `devices` to your project:
```xml
<importer plugin="ciao" name="devices">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
        <!-- Filter parameters -->
    </configuration>
</importer>
```
Currently, this importer retrieves all devices
#### Fields
| Field | Type | Description |
| --- | --- | --- |
| `id` | String | Unique id of the device |
| `externalId` | String | External id of the device |
| `description` | String | Description of the device |
| `type` | String | Type of the device, can be one of |
| | | - UNKNOWN |
| | | - DATAFOX |
| | | - MOBILE |
| `timeZone` | String | IANA Timezone id of the device |
##### Device action fields
For every device action associated with the device, fields with the name prefix `action[<index>]` will be added. For example, the `id` of first device action would be `action[0].id`
| Field | Type | Description |
| --- | --- | --- |
| `action[<index>].id` | i32 | Id of the action |
| `action[<index>].deviceActionId` | i32 | Device specific id of the action, usually an index of the position of the button or similar |
| `action[<index>].icon` | String | Name of the icon of the action |
| `action[<index>].description` | String | Description of the action |
| `action[<index>].type` | String | Type of the action, can be CLOCK or CUSTOM. Fields for the configuration are appended by either `clock` or `custom`, depending on the value of this field |
| `action[<index>].configuration.clock.timeTypeId` | String | Time type id for the clock action |
| `action[<index>].configuration.clock.costCenterId` | String | Costcenter id for the clock action |
| `action[<index>].configuration.clock.projectId` | String | Project id for the clock action |
| `action[<index>].configuration.clock.projectTaskId` | String | Project task id for the clock action |
| `action[<index>].configuration.custom.operation` | String | Operation of the custom action (device interprets the `operation` to perform specific operations)|

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

#### Fields
| Field | Type | Description |
| --- | --- | --- |
| `id` | String | Unique id of the project |
| `externalId` | String | External id of the project |
| `name` | String | Descriptive name of the project |
| `startDate.timeUtc` | Start date of the project in millis since UNIX Epoch |
| `startDate.timeZome` | IANA timezone id of the start date of the project in millis since UNIX Epoch |
| `endDate.timeUtc` | End date of the project in millis since UNIX Epoch |
| `endDate.timeZome` | IANA timezone id of the end date of the project in millis since UNIX Epoch |
| `closedDate.timeUtc` | Closed date of the project in millis since UNIX Epoch |
| `closedDate.timeZome` | IANA timezone id of the closed date of the project in millis since UNIX Epoch |

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
#### Parameters
This importer supports the additional filter parameters:
| Key | Description | Type | Default |
| --- | --- | --- | --- |
| `filter.absence` | Only return time types, that match the value given here for the time type option `absence` | bool | \<None> |
| `filter.bookable`| Only return time types, that match the value given here for the time type option `bookable` | bool | \<None> |

#### Fields
| Field | Type | Description |
| --- | --- | --- |
| `id` | String | Unique id of the time type |
| `description` | String | Descriptive name of the time type |
| `shorthand` | String | Shorthand of the time type |
| `color.alpha` | i32 | Alpha of the color (if available) forthe time type |
| `color.red` | i32 | Red of the color (if available) forthe time type |
| `color.green` | i32 | Green of the color (if available) forthe time type |
| `color.blue` | i32 | Blue of the color (if available) forthe time type |
| `icon` | String | The name of the icon for the time type |
| `options.absence` | bool | If available, it is `true` if the time type is an absence |
| `options.bookable` | bool | If available, it is `true` if the time type is bookable |


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
#### Parameters

This importer supports the additional filter parameters:
| Key | Description | Type | Default |
| --- | --- | --- | --- |
| filter.timeRange.startTime | Only return clock entries after this date/time | ISO 8601 | \<None> |
| filter.timeRange.endTime | Only return clock entries before this date/time | ISO 8601 | \<None> |
| filter.userId | Only return clock entries for this user id | UUID | \<None> |
| filter.creatorId | Only return clock entries created by this user | UUID | \<None> |
| filter.timeTypeId | Only return clock entries for this time type | UUID | \<None> |

#### Fields
| Field | Type | Description |
| --- | --- | --- |
| `id` | String | Unique id of the clock entry |
| `identitiy.userId` | String | User id of the clock entry |
| `identitiy.badgeId` | String | Badge id of the clock entry |
| `timestamp.timeUtc` | i64 | Time in millis (UTC) of the clock entry |
| `timestamp.timeZone` | String | IANA time zone id of the clock entry |
| `deviceId` | String | Device id of the clock entry |
| `timeTypeId` | String | Time type id of the clock entry |
| `projectId` | String | Project id of the clock entry |
| `projectTaskId` | String | Project task id of the clock entry |
| `costCenterId` | String | Cost center id of the clock entry |

## Exporter
### Accounts
This exporter creates an account on the CIAO backend using the `AccountService.create` gRPC service.
To use it, define a process and add the exporter with the name `accounts` to your project:
```xml
<exporter plugin="ciao" name="accounts">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
    </configuration>
</exporter>
```

#### Fields
This exporter uses the following fields from the record passed to it

| Field | Type | Description |
| --- | --- | --- |
| `id` | String | The id of the new account |
| `email` | String | The email address of the new account |
| `password` | String | The password for the new account |
| `avatar.id` | String | The id of the avatar image of the new account |
| `avatar.updatedAt.timeUtc` | String | The time in millis since Unix epoch of the last update of the image of the new account |
| `avatar.updatedAt.timeZone` | String | The The IANA timezon id of the last update of the image of the new account |
| `address.city` | String | The city of the new account |
| `address.postalCode` | String | The postalcode of the new account |
| `address.addressLine1` | String | The first address line of the new account |
| `address.addressLine2` | String | The second address line of the new account |
| `address.regionCode` | String | The region code of the new account |
| `address.state` | String | The state/province of the new account |
| `name.first` | String | The first name of the new account |
| `name.middle` | String | The middle name of the new account |
| `name.last` | String | The last name of the new account |

### Clock entries
This exporter creates a clock entry on the CIAO backend using the `TimeTrackingService.clock` gRPC service.
To use it, define a process and add the exporter with the name `clock_entries` to your project:
```xml
<exporter plugin="ciao" name="clock_entries">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
    </configuration>
</exporter>
```

#### Fields
This exporter uses the following fields from the record passed to it

| Field | Type | Description |
| --- | --- | --- |
| `timestamp.timeUtc` | String | The timestamp of the clock record in millis since Unix epoch |
| `timestamp.timeZone` | String | IANA time zone id of the timestamp |
| `identity.userId` | String | The user-id of the person clocking. If not given, a `identity.badgeId` must be provided |
| `identity.badgeId` | String | The badge-id of the person clocking. If not given, a `identity.userId` must be provided |
| `deviceId` | String | (optional) The device id of the clock record |
| `timeTypeId` | String | (optional) The time type id of the clock record |
| `projectId` | String | (optional) The project id of the clock record |
| `projectTaskId` | String | (optional) The project task id of the clock record |
| `costcenterId` | String | (optional) The cost center id of the clock record |

### Projects
This exporter creates projects on the CIAO backend using the `ProjectService.clock` gRPC service.
To use it, define a process and add the exporter with the name `projects` to your project:
```xml
<exporter plugin="ciao" name="projects">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
    </configuration>
</exporter>
```

#### Fields
| Field | Type | Description |
| --- | --- | --- |
| `id` | String | Unique id of the project |
| `externalId` | String | External id of the project |
| `name` | String | Descriptive name of the project |
| `startDate.timeUtc` | Start date of the project in millis since UNIX Epoch |
| `startDate.timeZome` | IANA timezone id of the start date of the project in millis since UNIX Epoch |
| `endDate.timeUtc` | End date of the project in millis since UNIX Epoch |
| `endDate.timeZome` | IANA timezone id of the end date of the project in millis since UNIX Epoch |
| `closedDate.timeUtc` | Closed date of the project in millis since UNIX Epoch |
| `closedDate.timeZome` | IANA timezone id of the closed date of the project in millis since UNIX Epoch |

### Project tasks
This exporter creates a project task on the CIAO backend using the `ProjectTaskService.create` gRPC service.
To use it, define a process and add the exporter with the name `project_tasks` to your project:
```xml
<exporter plugin="ciao" name="project_tasks">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
    </configuration>
</exporter>
```

#### Fields
This exporter uses the following fields from the record passed to it

| Field | Type | Description |
| --- | --- | --- |
| `id` | String | The id of the new project task |
| `projectId` | String | The project id of the new project task |
| `name` | String | The name of the new project task |


### Cost centers
This exporter creates a cost center on the CIAO backend using the `CostCenterService.create` gRPC service.
To use it, define a process and add the exporter with the name `cost_centers` to your project:
```xml
<exporter plugin="ciao" name="cost_centers">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
    </configuration>
</exporter>
```

#### Fields
This exporter uses the following fields from the record passed to it

| Field | Type | Description |
| --- | --- | --- |
| `id` | String | The id of the new cost center |
| `name` | String | The name of the new cost center |
