# rite - Rust Import/Transform/Export
rite is a framework for importing data from a datasource, optionally transform the data, and then exporting it to a datasink
For easy usage, there is a shell script to run the container. See [the documentation for the script](rite.sh.md)

## model
This [library](model/README.md) describes the data model of the framework

## import
This [library](import/README.md) defines the traits for the import interface

## transform
This [library](transform/README.md) defines the traits for the transformer interface

## export
This [library](export/README.md) defines the traits for the export interface

## plugin
A [library](plugin/README.md) for managing import/transfom/export dynamic libraries

## plugins
A set of dynamic libraries to be used as [plugins](plugins/README.md) for rite

## helper
A [library](helper/README.md) with some useful functions

## examples
Some [example](examples/README.md) implementations for plugins

## data
Test files and [example rite configurations](data/README.md)