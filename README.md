# chronox
A simple Rust utility to convert log files from local time to UTC 🦀.


It takes dates in `Month Day HH:MM:SS` format and converts it to `YYYY-MM-DD HH:MM:SS Z` format.

For example using a -06:00 offset:

```
Aug 27 00:00:00 host systemd[1]: Reloaded httpd.service - The Apache HTTP Server.
```

Gets converted to:
```
2023-08-27 06:00:00 UTC host systemd[1]: Reloaded httpd.service - The Apache HTTP Server.
```

# Usage
`chronox` can be used from the command line with the following syntax:

```bash
chronox [logfile] [timezone offset]
```

The timezone offset needs to be in `-+00:00` format.

A file with the suffix `-chronox` will be saved in the same directory as the original file.

# Downloading

Grab one of the binaries from the [releases page](https://github.com/msLinuxNinja/chronox/releases/tag/0.1.0) for the corresponding Operating System.

Or download directly by running:

```bash
wget https://github.com/msLinuxNinja/chronox/releases/download/0.1.0/chronox
chmod +x chronox
```

# Gotchas

* The utility assumes the year is current year.
* Only files that use `Month Day HH:MM:SS` format would work (Like `messages` or `syslog`).