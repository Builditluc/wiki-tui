You can use the CLI arguments to further fasten up your wikipedia experience in
wiki-tui. While there are more QOL-features and arguments for the CLI planned,
those are the things you can currently do with the CLI:

- [Faster search](#search-query-argument)
- [Change the language](#changing-the-language)
- [Print cache and config path](#print-cache-and-config)
- [Adjust the logging level on the fly](#change-logging-level)

!!! tip
    You can always run `wiki-tui --help` to get an overview of available 
    arguments

### Search Query Argument

```
wiki-tui <search-query>
```

!!! example
    If you already know that you want to search for `github` you can pass it in as a argument

    ```
    wiki-tui github
    ```

    It will open wiki-tui and already start the search so you don't have to type it into the search bar

### Changing the language

```
wiki-tui --language <language-code>
```

!!! example
    If you want to change the language to German, simply pass it in as a argument
    
    ```
    wiki-tui --language de
    wiki-tui --language german
    wiki-tui --language deutsch
    ```

    It will open wiki-tui with the language set to German

!!! tip
    You can also use the short variant of this argument `-l` to change the language
    
    ```
    wiki-tui -l de
    ```

For more information about language changing, view [this page](../configuration/api.md#supported-languages) of the docs

### Print cache and config

To view the cache directory where the log file and crash-reports are written to,
use this command

```
wiki-tui --cache-dir
```

If you want to instead view the path for the config file, use this command

```
wiki-tui --config-path
```

### Change logging level

You can also change the configured logging level. 

```
wiki-tui --level <level>
```

!!! example
    Changing the level to `Debug`

    ```
    wiki-tui --level 0
    ```

Available values are:

| Value | Level | 
|---|-------|
| 0 | Debug | 
| 1 | Info  |
| 2 | Warn  |
| 3 | Error |
