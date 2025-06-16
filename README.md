[![Coverage](https://img.shields.io/badge/coverage-18%25-red)](https://ciao-systems.github.io/rite/tarpaulin-report.html)

# rite - Rust Import/Transform/Export
rite is a framework for importing data from a datasource, optionally transform the data, and then exporting it to a datasink
For easy usage, there is a shell script to run the container. See [the documentation for the script](rite.sh.md)

## Overview
### model
This [library](https://github.com/CIAO-systems/rite-lib-model) describes the data model of the framework

### import
This [library](https://github.com/CIAO-systems/rite-lib-import) defines the traits for the import interface

### transform
This [library](https://github.com/CIAO-systems/rite-lib-transform) defines the traits for the transformer interface

### export
This [library](https://github.com/CIAO-systems/rite-lib-export) defines the traits for the export interface

### plugin
A [library](https://github.com/CIAO-systems/rite-lib-plugin) for managing import/transfom/export dynamic libraries

### helper
A [library](https://github.com/CIAO-systems/rite-lib-helper) with some useful functions

### examples
Some [example](examples/README.md) implementations for plugins
## Creating a plugin
To get started creating a RITE plugin, read the [How to write a plugin guide](write-a-plugin.md)

## Plugins
### base
The base executable and common plugins. See the [README](base/README.md)

### extended
A set of dynamic libraries to be used as [plugins](extended/README.md) for rite

### custom
A set of dynamic libraries to be used as [plugins](custom/README.md) for rite

## Building
### git submodules
Some of the projects use git submodules. To initialize all of them, including 
nested submodules, execute the following command:
```bash
git submodule update --init --recursive
```
#### Branch
To make sure, that the submodule uses the `HEAD` of the `main` branch, execute this (replacing <submodule-path> with the actual path of the submodule):
```bash
git submodule set-branch -b main -- <submodule-path>
```
To update the submodule to the latest commit in the branch, execute:
```bash
git submodule update --remote --merge
```