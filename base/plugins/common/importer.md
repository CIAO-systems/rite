# env
This importer reads all environment variables. 
To use it, define a process and add the importer with the name `env` to your project:
```xml
<importer plugin="common" name="env">
</importer>
```
The resulting records will have the following fields:
| Field | Description |
| --- | --- |
| name | The name of the OS environment variable |
| value | The value of the OS environment variable |

