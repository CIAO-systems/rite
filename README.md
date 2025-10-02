[![Coverage](https://img.shields.io/badge/coverage-93%25-green)](https://ciao-systems.github.io/rite/tarpaulin-report.html)

# rite - Rust Import/Transform/Export
rite is a framework for importing data from a datasource, optionally transform the data, and then exporting it to a datasink

Go to the [user documentation](doc/user-doc.md) to learn how to create a rite import/export.

## Overview
### model
This [library](https://github.com/CIAO-systems/rite-lib-model) describes the data model of the framework

### examples
Some [example](examples/README.md) implementations for plugins

## Creating a plugin
To get started creating a RITE plugin, read the [How to write a plugin guide](doc/write-a-plugin.md)

## Plugins
### base
The base executable and common plugins. See the [README](base/README.md)

### extended
A set of dynamic libraries to be used as [plugins](extended/README.md) for rite

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
To reset the project and all submodules:
```bash
git reset --hard --recurse-submodules
```