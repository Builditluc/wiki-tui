When a crash in wiki-tui occurrs you will see the following output being shown in your terminal.

<figure markdown>
![Crash Message](../assets/images/crash-1.png)
</figure>

You will then see a crash report in the cache directory (which can be found by running `wiki-tui --cache-dir`). It contains:

- the version of wiki-tui you are currently using
- your operating system 
- where and why the crash occured
- the log file (if available)

The name of the report-file is generated like this

```
wiki_tui-crash_report-{UUID}.log
```

Next, please [file a bug report](../contributing/index.md) and upload the crash report so we can work on fixing the problem.
