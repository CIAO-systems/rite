# Mapper transformer configuration
The mapper transformer maps a source value to a target value. For example, 
this is required, if a value from the source needs to be converted to another value, that represents the counterpart for this value in the target. In an import/export from one CRM to another CRM, the source value for the gender might be only the first letter (e.g. `F`), but the target CRM expects a full string, like `female`. This is where a mapper can be used:
```xml
<mapper>
   <field>
     <name source="gender_code" target="gender" />
     <type source="string" target="string" />
     <patterns>
        <!-- RegEx patterns will be applied in the order of appearance -->
        <pattern matcher="^(.*)" replacement="PREFIX: $1" />
     </patterns>
     <values>
        <value source="M" target="male" />
        <value source="F" target="female" />
        <value source="D" target="other" />
     </values>
   </field>
</mapper>
```
# Elements
A mapper configuration consists of a list of field definitions.
## Field
For the mapping, the transformer takes the current record and tries to find a field with the `name.source`. If such a field is found, it tries to find a value in the list of values, that matches one of the values in the field defintion. 
If there is such a value, in the resulting record a field with the `target.name` will be created, with the type of `type.target` and the `value.target` of the matching value.

A field definition has the elements `name`, `type` and `patterns` or `values`:
```xml
   <field>
     <name source="<source-name>" target="<target-name>" />
     <type source="<source-type>" target="<target-type>" />
     <patterns>
        <!-- Patterns definitions -->
     </patterns>

     <values>
        <!-- Value definitions -->
     </values>
   </field>
```
If `patterns` and `values` are provided, only `values` will be processed.

### Name
The name definition tells the mapper, what name the field in the source has, and what name it should have in the resulting target. 
### Type
The type definition tells the mapper, what type the field in the source has, and what type it should have in the resulting target. 
The following types are supported:
* `string`: A String
* `i32`: A 32-bit signed integer
* `i64`: A 64-bit signed integer
* `f32`: A 32-bit signed float
* `f64`: A 64-bit signed float

### Values
A list of values that should be mapped `value.source` -> `value.target`. If the types of source and target are different, a conversion will be done. If the conversion is not possible. The transformer will add a field with the Value::None in the target record. For information about the error, have a look in the logs.
```xml
   <field>
     <name source="name" target="newname" />
     <type source="string" target="string" />
     <values>
        <value source="Samuel Clemens" target="Mark Twain" />
        <value source="Farrokh Bulsara" target="Freddy Mercury" />
        <value source="Norma Jean Mortenson" target="Marilyn Monroe" />
        <value source="Cassius Clay" target="Muhammad Ali" />
     </values>
   </field>
```

### Patterns
A list of regular expression replacement patterns. The replacement patterns will be applied in order of apearance in the configuration file. The mappe applies the `matcher` regular expression on the source field value and will replace all found parts with the value from the `replacement`.
For more information on regular expressions, go to the [MDN](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Regular_expressions)

#### Matcher
This is a regular expression. Some example expressions:
| RegEx | Description |
| --- | --- |
| "-" | Replaces the character `-` with an empty string (removing all dashes) |
| "^(.*)" | The expression takes the whole text at the beginning |
| "(.*)$ |  The expression takes the whole text at the end |
| "([A-Z])\\w+" | Find all words that start with a capital letter |

#### Replacement
All found occurences will be replaced with this value. There are varaibles that can be used in the text. 
| Variable | Description |
| --- | --- |
| $n | The captured group n 
| ${<field_name>} | Replaces the text with a field from the record currently processed.

### Examples
```xml
<mapper>
   <field>
     <name source="title" target="title" />
     <type source="string" target="string" />
     <patterns>
        <!-- Patterns will be applied in the order of appearance -->
        <pattern matcher="^(.*)" replacement="Episode ${episode_id}: $1 (${release_date})" />
     </patterns>
   </field>
</mapper>
```
In the above example, the field `title` from the record will be replaced with a combination of other fields of the record and the original value ($1 is the first group found by the regex)

#### Adding a prefix
```xml
   <pattern matcher="^(.*)" replacement="PREFIX-$1" />
```
#### Adding a suffix
```xml
   <pattern matcher="(.*)$" replacement="$1-SUFFIX" />
```
#### Removing a character
```xml
   <pattern matcher="-" replacement="" />
```
#### Combining field values
```xml
   <pattern matcher="" replacement="${firstname} ${lastname}" />
```
