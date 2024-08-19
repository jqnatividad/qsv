# Security Policy

qsv is designed to be secure by default. It is built with security in mind, and it is continuously tested for security vulnerabilities.

It uses a lot of third-party libraries, all of which are inspected for security vulnerabilities with `cargo-audit`. The libraries are updated regularly to ensure that the latest security patches are applied.

qsv allows `unwrap`,`expect` and `unsafe` in the codebase for performance reasons, but they are always accompanied by a neighboring `// safety:` comment with the justification (typically, to skip redundant bounds checking in performance-sensitive, "hot" loops).

## Supported Versions

qsv has a [very rapid release tempo](https://github.com/jqnatividad/qsv/releases), with several releases/month.   
It has a built-in self-update engine that makes it very convenient to upgrade to the latest version.

## Reporting a Vulnerability

If you've found a vulnerability, please [create an issue](https://github.com/jqnatividad/qsv/issues/new/choose) in GitHub.

However, if a vulnerability is severe and could lead to zero-day exploits, please send an email to [qsv-severe@datHere.com](mailto:qsv-severe@datHere.com) instead.

The vulnerability report will be addressed within one business day. A  mitigation/workaround, if available, will be included in the next release and mentioned in the changelog, with zero-day mitigations only being mentioned only when it's been fully fixed.
