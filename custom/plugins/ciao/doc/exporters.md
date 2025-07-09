# Absences
This exporter creates an absence on the CIAO backend using the `AbsenceService.create` gRPC service.
To use it, define a process and add the exporter with the name `absences` to your project:
```xml
<exporter plugin="ciao" name="absences">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
    </configuration>
</exporter>
```
## Fields
This exporter uses the following fields from the record passed to it

| Field | Type | Description |
| --- | --- | --- |
| `startDate` | String | The start date of the absence as string with the format YYYY-MM-DD |
| `endDate` | String | The end date of the absence as string with the format YYYY-MM-DD |
| `startHalfDay` | bool | `true` if the start day is a half day absence |
| `endHalfDay` | String | `true` if the end day is a half day absence |
| `timeTypeId` | String | The time type id for the new absence |
| `userId` | String | The user id for the new absence |


# Accounts
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

## Fields
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

# Clock entries
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

## Fields
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

# Projects
This exporter creates projects on the CIAO backend using the `ProjectService.create` gRPC service.
To use it, define a process and add the exporter with the name `projects` to your project:
```xml
<exporter plugin="ciao" name="projects">
    <configuration>
        <config key="url" value="$CIAO_URL" />
        <config key="api-key" value="$CIAO_API_KEY" />
    </configuration>
</exporter>
```

## Fields
| Field | Type | Description |
| --- | --- | --- |
| `id` | String | Unique id of the project |
| `externalId` | String | External id of the project |
| `name` | String | Descriptive name of the project |
| `startDate.timeUtc` | i64 | Start date of the project in millis since UNIX Epoch |
| `startDate.timeZome` | String | IANA timezone id of the start date of the project in millis since UNIX Epoch |
| `endDate.timeUtc` | i64 | End date of the project in millis since UNIX Epoch |
| `endDate.timeZome` | String | IANA timezone id of the end date of the project in millis since UNIX Epoch |
| `closedDate.timeUtc` | i64 | Closed date of the project in millis since UNIX Epoch |
| `closedDate.timeZome` | String | IANA timezone id of the closed date of the project in millis since UNIX Epoch |

# Project tasks
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

## Fields
This exporter uses the following fields from the record passed to it

| Field | Type | Description |
| --- | --- | --- |
| `id` | String | The id of the new project task |
| `projectId` | String | The project id of the new project task |
| `name` | String | The name of the new project task |


# Cost centers
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

## Fields
This exporter uses the following fields from the record passed to it

| Field | Type | Description |
| --- | --- | --- |
| `id` | String | The id of the new cost center |
| `name` | String | The name of the new cost center |
