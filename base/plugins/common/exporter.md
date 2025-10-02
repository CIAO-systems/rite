# Console
The `console` exporter prints the record on standard output. 
```xml
    <exporters>
        <exporter plugin="common" name="console">
            <configuration>
                <config key="prefix" value="<prefix-to-print-before-record>" />
                <config key="postfix" value="<postfix-to-print-after-record>" />
                <config key="separator" value="&#xA;" /> 
                <config key="field-prefix" value=" " />
            </configuration>
        </exporter>    
    </exporters>
```
## Configuration
| Key | Description | Default |
| --- | --- | --- |
| prefix | The optional configuration `prefix` can be used, to print a leading text.| |
| postfix | The optional configuration `postfix` can be used, to print a trailing text.  | |
| separator | The optional configuration `separator` can be used, to print an alternative separator for the fields. | ", " |
| field-prefix | The optional configuration `field-prefix` can be used, to print an additional text before each field | |

Every record will then be printed, with the fields separated by the `separator`:
```xml
<prefix>
<field-prefix>record.field1=record.value1<separator><field-prefix>record.field2=record.value2
<postfix>
```
To print each field indented by some spaces on separate lines and brackets around each record, you can use this configuration:
```xml
    <config key="prefix" value="(" />
    <config key="postfix" value=")" />
    <config key="separator" value="&#xA;" /> 
    <config key="field-prefix" value="   " />
```
This will result in an output like this:
```
(
    record1.field1=record1.value1
    record1.field2=record1.value2
)
(
    record2.field1=record2.value1
    record2.field2=record2.value2
)
```
# Template
The `template` exporter takes all records and at the end of the import passes the records to the [Tera](https://keats.github.io/tera/docs/#templates) template engine.

```xml
    <exporters>
        <exporter plugin="common" name="template">
            <configuration>
                <config key="templateFile" value="<path-of-the-template-file>" />
                <config key="outputFile" value="<path-of-the-outputfile>" />
            </configuration>
        </exporter>    
    </exporters>
```
## Configuration
| Key | Description | 
| --- | --- |
| templateFile | The pathname of the template file, which prints the records |  
| outputFile | The pathname of the output file. After the template has been rendered, the result will be written into this file. The file will be overwritten |  

## Variables for the template
The plugin will provide an array of all records from the input. The name of the array in the template is `records`. To process them, a loop over records can be done like this:
```
{% for record in records %}
    {{ record.id }}
{% endfor %}
```
In the above example, the record is assumed to have a field with the name `id`. To access any field in a record, the notation `record.<fieldname>` can be used.
For a detailed documentation of all the features of the Tera template language, have a look at [their documentation](https://keats.github.io/tera/docs/#templates)