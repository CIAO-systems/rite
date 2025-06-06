# LLM (Large Language Model) plugin
This plugin provides an importer, that generates records by asking an AI chatbot.
To use the importer, include the plugin in the rite-XML:
```xml
<rite>
    <plugins>
        <!-- If the plugin is not in the LD_LIBRARY_PATH, provide a "path" attribute -->
        <plugin id="llm" name="rite_llm"/>
    </plugins>
</rite>
```
## Importer
All importers share some common configuration options:
| **Configuration key** | **Description** | **Required** | **Default** |
| --- | --- | --- | --- |
| agent | The name of the agent to be used | Yes | 
| prompt-file | A filename (text file) that contains the prompt for the chat. If this configuration is set, then `prompt` will be ignored | No, if `prompt` is set | 
| prompt | The prompt for the chat (only used, if `prompt-file` is not set) | Yes, if `prompt-file` is not set |


### Ollama
To use a locally installed LLM, [ollama](https://ollama.com/) can be used. In the configuration add an importer with the name `ollama`:
```xml
<importer plugin="llm" name="ollama">
```
### Example
```xml
<rite>
    <plugins>
        <plugin id="common" name="rite_common"/>
        <plugin id="llm" name="rite_llm"/>
    </plugins>
    <processes>
        <process id="llm.ollama">
            <importer plugin="llm" name="ollama">
                <configuration>
                    <config key="agent" value="qwen2.5:7b" />
                    <config key="prompt-file" value="$RITE_CONFIG_PATH/prompt.txt" />
                </configuration>
            </importer>
            <exporters>
                <exporter plugin="common" name="console" />
            </exporters>
        </process>
    </processes>
</rite>
```

#### Example prompt file
```text
List ten random German last and first names and add a random age.
Fieldnames should be 'age', 'first' and 'last'
```

##### Output
Since the system prompt instructs the model to always return a JSON array of 
record, the output for the above configuration might look something like this:
```
age=37, first=Max, last=Mustermann
age=29, first=Franz, last=Hansen
age=45, first=Sabine, last=Meier
age=53, first=Egon, last=Schlomer
age=61, first=Helga, last=Kremer
age=31, first=Johannes, last=Müller
age=24, first=Lena, last=Werner
age=48, first=Uwe, last=Neumann
age=57, first=Monika, last=Stenzel
age=39, first=Hans, last=Klein
```

### Configuration
| **Configuration key** | **Description** | **Required** | **Default** |
| --- | --- | --- | --- |
| url | The URL for the Ollama service | No | http://localhost:11434

