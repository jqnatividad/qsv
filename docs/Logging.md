# Logging

To keep things simple:

* no additional logging config file required
* logging level set via env var (`QSV_LOG_LEVEL`)
* logging can be disabled completely
* logging goes to console (stderr) only
* only requires standard Log trait to add log traces in code

## Enable Logging

Set enviroment variable `QSV_LOG_LEVEL` to desired level. Default is `off`.
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
[2021-11-29T10:12:43.430Z ERROR qsv] failed to open /home/mhuang/tmp/csv/Data7602DescendingYearOrder.cs: No such file or directory (os error 2)
```

__enable WARN for all, and DEBUG for _count_ command only__

```
$ QSV_LOG_LEVEL=warn,qsv::cmd::count=debug qsv count ~/tmp/csv/Data7602DescendingYearOrder.cs
[2021-11-29T10:10:12.053Z DEBUG qsv::cmd::count] input: "/home/mhuang/tmp/csv/Data7602DescendingYearOrder.cs", no_header: false, delimiter: None
[2021-11-29T10:10:12.053Z ERROR qsv] failed to open /home/mhuang/tmp/csv/Data7602DescendingYearOrder.cs: No such file or directory (os error 2)
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