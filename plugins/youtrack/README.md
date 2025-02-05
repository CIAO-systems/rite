# YouTrack RITE plugin
A plugin with importers for [YouTrack](https://www.jetbrains.com/youtrack/).
To use one of the importers, include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="youtrack" name="rite_youtrack"/>
    </plugins>
</rite>
```
## Importers
### Generic
With this importer, all resources, that the YouTrack REST API offers can be processed.
To use it, define a process and add the importer **without** any name to your project:
```xml
<importer plugin="youtrack">
    <configuration xml="$RITE_CONFIG_PATH/users.xml">
        <config key="url" value="https://ciao-systems.youtrack.cloud" />
        <config key="token" value="$YOUTRACK_TOKEN" />
    </configuration>
</importer>
```
#### Configuration
The following table shows all the `config` values, that are supported:

| Key | Description | Required | 
|---|---|---|
| `url` | Specifies the YouTrack instance URL. | Yes |  
| `token` | Defines the YouTrack personal access token for authentication. | Yes | 
#### Generic XML configuration
For the generic importer, there is a special XML configuration availabe for
filtering the amount of data requested.
##### Example
```xml
<rite-youtrack-import>
    <dataset path="users"> 
        <fields>
            <field id="id" />
            <field id="login" />
            <field id="email" />
            <field id="fullName" />
        </fields>
    </dataset>
</rite-youtrack-import>
```

##### Dataset elements
| Element | Description | Required | Data Type |
|---|---|---|---|
| `path` | The path. It will be appended as given to the base URL | Yes | String |
| `resource`  | If a special item instead of a list of items should be queried, `resource` can be provided | No | String |
| `query`  | An optional [YouTrack query](https://www.jetbrains.com/help/youtrack/cloud/search-and-command-attributes.html?Search-and-Command-Attributes) expression to filter the data set. | No | String |
| `sub_resource`  | When an entity has sub-resource, this can be used to get those | No | String |
| `fields`  | A list of field elements. Each field element has a mandatory attribute `id` which is one attribute of a YouTrack entity | No | String |

#### Output
The importer creates records with all the fields requested. The type is according to the type of the YouTrack entity attribute. If there is a composite field (for example a [User](https://www.jetbrains.com/help/youtrack/devportal/api-entity-User.html) field), the values of the entity will be added as field with a name of `entity.attribute`. For example, the `reporter` of an issue that is requested with the field id `reporter(id,email)` will add two fields to the resulting record: `reporter.id` and `reporter.email`.



### Time
This special importer reads the [work items](https://www.jetbrains.com/help/youtrack/devportal/resource-api-workItems.html) and provides records with the most 
relevant data.
To use it, define a process and add the importer with the name `time` to your project:
```xml
<importer plugin="youtrack" name="time">
    <configuration xml="$RITE_CONFIG_PATH/time.xml">
        <config key="url" value="https://ciao-systems.youtrack.cloud" />
        <config key="token" value="$YOUTRACK_TOKEN" />
    </configuration>
</importer>
```
#### Configuration
The following table shows all the `config` values, that are supported:

| Key | Description | Required | 
|---|---|---|
| `url` | Specifies the YouTrack instance URL. | Yes |  
| `token` | Defines the YouTrack personal access token for authentication. | Yes | 

#### Time-Tracking XML configuration
For the time tracking importer, there is a special XML configuration availabe for
filtering the amount of data requested.
##### Attributes
| Attribute | Description | Required | Data Type | Default |
|---|---|---|---|---|
| `start-date` | The start date of the time period for data import. | No | String (YYYY-MM-DD) | None (no start date limit) |
| `end-date` | The end date of the time period for data import. | No | String (YYYY-MM-DD) | None (no end date limit) |
##### Example
```xml
<rite-youtrack-import-time>
    <!-- Dates in format YYYY-MM-DD, empty for open interval -->
    <time-tracking
        start-date="2025-01-01"
        end-date="2025-01-08"/>
</rite-youtrack-import-time>
```
#### Output
The importer creates records with the following fields
| Field | Type | Description |
|---|---|---|
| minutes  | i32  | Number of minutes spent for the work item  |
| created  | i64  | Date and time when the work item was created (in unix time millis)  |
| email | String  | E-Mail address of the author of the work item |
| issue   | String  | The readable id of the issue for this work item  |
| issue.summary | String  | The summary for the issue  |
| project  | String  | The project id of the project for the issue of the work item  |
| project.name | String  | The project name of the project for the issue of the work item |
