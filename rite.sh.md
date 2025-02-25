# Script to run a rite import/transform/export
The script takes a local file to run `rite` with it. Since there need to be more files for such a run, the script mounts the direcotry, where the given file is located into the container.
The script also makes sure, that a `logs` directory exists in the directory of the configuration file.

It can be downloaded from the [GitHub repository](https://github.com/CIAO-systems/rite/blob/main/rite.sh)

## Parameters
| Parameter | Description | Default |
| --- | --- | --- |
| -f, --file | The local rite configuration XML file | - |
| -ci, --container-image | An alternative container image | `ghcr.io/ciao-systems/rite:main` |
| -h, --help | Shows the usage of the script | - |
| --debug | When given, the container is started with a bash shell | `false` |
| --silent | When given, the script does not print anything | `false` |

## Example
```bash
$> ./rite.sh --file $HOME/tmp/rite/faker/rite.xml
```
In the above example, the script mounts the directory `$HOME/tmp/rite/faker/` to the container directory `/data` and creates a directory `$HOME/tmp/rite/faker/logs` and mounts it to the container directory `/app/logs`
The container then executes the `rite` executable with the file `/data/rite.xml` which is the local file `$HOME/tmp/rite/faker/rite.xml`

## log4rs Configuration
To configure the amount of logging, a `log4rs.yaml` file can be placed in the local directory. It will be used by `rite`.
