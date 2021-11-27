# Logging

## Requirements

Here's a rough spec of my ideal qsv logging implementation:

* off by default, turned on by setting environment variable QSV_LOGGING to (TRUE, 1, true, etc.)
* configuration is largely done with a YAML file file named qsv-log4rs.yml stored in the same directory as the binary and/or in the ~/.qsv directory
integrated into config.rs
* has some helper functions in utils.rs that wraps log4rs calls so they can be easily called from the different cmd programs.

## TODO

[ ] add logging crates
[ ] use env var QSV_LOGGING to enable
[ ] config.rs
[ ] util.rs
[ ] POC for count command 