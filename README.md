[![Coverage](https://img.shields.io/badge/coverage-18%25-red)](https://ciao-systems.github.io/rite/tarpaulin-report.html)

# rite - Rust Import/Transform/Export
rite is a framework for importing data from a datasource, optionally transform the data, and then exporting it to a datasink
For easy usage, there is a shell script to run the container. See [the documentation for the script](rite.sh.md)

## model
This [library](libraries/model/README.md) describes the data model of the framework

## import
This [library](libraries/import/README.md) defines the traits for the import interface

## transform
This [library](libraries/transform/README.md) defines the traits for the transformer interface

## export
This [library](libraries/export/README.md) defines the traits for the export interface

## plugin
A [library](https://github.com/CIAO-systems/rite-lib-plugin) for managing import/transfom/export dynamic libraries

## helper
A [library](https://github.com/CIAO-systems/rite-lib-helper) with some useful functions

## examples
Some [example](examples/README.md) implementations for plugins

## data
Test files and [example rite configurations](data/README.md)

# Plugins
## base
A set of dynamic libraries to be used as [plugins](base/plugins/README.md) for rite

## extended
A set of dynamic libraries to be used as [plugins](extended/plugins/README.md) for rite

## custom
A set of dynamic libraries to be used as [plugins](custom/plugins/README.md) for rite

# Building
## git submodules
Some of the projects use git submodules. To initialize all of them, including 
nested submodules, execute the following command:
```bash
git submodule update --init --recursive
```
### Branch
To make sure, that the submodule uses the `HEAD` of the `main` branch, execute this (replacing <submodule-path> with the actual path of the submodule):
```bash
git submodule set-branch -b main -- <submodule-path>
```
To update the submodule to the latest commit in the branch, execute:
```bash
git submodule update --remote --merge
```