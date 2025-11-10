# Logging Settings

wiki-tui writes everything it does (logs) into a so-called `logfile`. These logs are useful for debugging (so we as the developers can understand what wiki-tui was trying to do before it failed) and deliver some interesting insight into how wiki-tui works.
The log file gets written to the cache-dir which can be found by running `wiki-tui --cache-dir`. The file is named `wiki_tui.log`

## Enable / Disable logging entirely

[:octicons-tag-24: 0.3.0][release-0.3.0] ·
:octicons-milestone-16: Default: `true`

With this option, you can disable logging entirely if you don't want it.

```toml
logging.enabled = true
```

!!! note "About Logging"
    Logging helps us understand what wiki-tui was doing when it failed or more importantly why it failed. When wiki-tui crashes it puts the logs into the crash report and gives us more detailed crash information.

    **Why you should keep logging enabled**

    When a crash occurs or you encounter a bug, without the log file we cannot do very much if we cannot reproduce it. But even if we can reproduce your bug or crash, we can usually fix it much faster with the logs. 

    You don't need to have your `log_level` set to `INFO` (although, we would appreciate it :)). Even if you only have the `log_level` set to `WARN`, it helps us so much more than no logs at all. So please, for you it is only a single file that gets overwritten when you start wiki-tui again. But for us, it means so much more.

## Adjusting the Log Level

[:octicons-tag-24: 0.3.0][release-0.3.0] ·
:octicons-milestone-16: Default: `Info`

Changing the log level reduces the number of logs that get written to the log file. Higher levels mean only more severe logs get written. Available levels are:

| Level | Description | Note |
| ----- | ----------- | ---- |
| Debug | A log level that is used for events considered to be useful during the software debugging when more granular information is needed. | This slows down the application and should only be used for development |
| Info  | An event happened, the event is purely informative and can be ignored during normal operations. | The default and also recommended log level |
| Warn  | Unexpected behavior happened inside the application, but it is continuing its work and the key features are operating as expected. | |
| Error | Basically: Something has **really** gone wrong and the program only knows one way to get out of this, **crash** |  |

> Credit to [sematext.com](https://sematext.com/blog/logging-levels/) for the descriptions of the log levels

```toml
logging.log_level = "Info"
```

[release-0.3.0]: https://github.com/Builditluc/wiki-tui/releases/tag/v0.3
