# Logging

To keep things simple:

* no additional logging config file required
* logging level set via env vars (`QSV_LOG_LEVEL` and `QSV_LOG_DIR`)
* logging can be disabled completely (default behavior)
* logging goes to specified directory with auto log rotation (default: current directory)
* if specified directory doesn't exist, qsv will attempt to create it
* logs to `qsv_rCURRENT.log` - rotating at 1mb, keeping 10 most recent log files uncompressed, 
  and 100 gz-compressed log files named sequentially (e.g. qsv_0001.log, qsv0010.log.gz)
* only requires standard Log trait to add log traces in code

## Enable Logging

Set environment variable `QSV_LOG_LEVEL` to desired level. Default is `off`.
* off - no logging
* error
* warn
* info
* trace
* debug

### Examples

__enable WARN for all modules__
```
$ QSV_LOG_LEVEL=warn qsv count ~/tmp/csv/Data7602DescendingYearOrder.cs

# Last line in qsv_rCURRENT.log file
[2021-12-05 09:10:18.976948 -05:00] ERROR [qsv] src\main.rs:233: failed to open /home/mhuang/tmp/csv/Data7602DescendingYearOrder.cs: The system cannot find the file specified. (os error 2)
```

__enable WARN for all, and DEBUG for _count_ command only__

```
$ QSV_LOG_LEVEL=warn,qsv::cmd::count=debug qsv count ~/tmp/csv/Data7602DescendingYearOrder.csv
# Last two lines in qsv_rCURRENT.log file
[2021-12-05 09:14:46.496051 -05:00] DEBUG [qsv::cmd::count] src\cmd\count.rs:37: input: "/home/mhuang/tmp/csv/Data7602DescendingYearOrder.csv", no_header: false, delimiter: None 
[2021-12-05 09:14:46.498044 -05:00] ERROR [qsv] src\main.rs:233: failed to open /home/mhuang/tmp/csv/Data7602DescendingYearOrder.csv: The system cannot find the file specified. (os error 2)
```

__enable INFO to log invocations and elapsed time__

```
$ QSV_LOG_LEVEL=info qsv count 311-10k.csv
# Last two lines of qsv_rCURRENT.log file
[2021-12-01 10:28:28.925011 -05:00] INFO [qsv] src\main.rs:177: START: count .\311-10k.csv
[2021-12-01 10:28:28.997835 -05:00] INFO [qsv] src\main.rs:215: END elapsed: 0.0729002
```

## Add log traces

Just use the Log trait macros!

See _count_ command as example.

```
use log::{debug, info};

<snip>
    debug!("input: {:?}, no_header: {}, delimiter: {:?}", 
            (&args.arg_input).clone().unwrap(),
            &args.flag_no_headers,
            &args.flag_delimiter
        );
<snip>
```
