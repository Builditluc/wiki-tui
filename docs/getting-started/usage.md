So you've installed wiki-tui, great! (if not, please check the [installation guide](./installation.md)) This page provides an overview of how to use wiki-tui. If you already feel comfortable with the program, check how 
you can configure wiki-tui to your needs.

!!! info
    wiki-tui performs its actions synchronously. That means that you have to wait a few seconds after starting
    the search or opening an article until the window updates. Any keys you press in this wait time will be 
    send after the action finishes

## Startup and CLI

You can start wiki-tui by running the following command from you terminal

```
wiki-tui
```

This will run the program and you can start browsing wikipedia. If you want, you can use the CLI to fasten up your search.

!!! tip
    You can always run `wiki-tui --help` to get an overview of available arguments

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
    If you want to change the language to German, simply pass its language code `de` as a argument
    
    ```
    wiki-tui --language de
    ```

    It will open wiki-tui with the language set to German


## Keybindings and Controls

This is a list of all available actions and what the default keybinding for that action is. Configurable
actions can be remapped in the config

| Action         | Default Keybinding | Configurable?    |
| -------------- | ------------------ | ---------------- |
|  `UP`          | ++up++             | :material-check: |
|  `DOWN`        | ++down++           | :material-check: |
|  `LEFT`        | ++left++           | :material-check: |
|  `RIGHT`       | ++right++          | :material-check: |
|  `NEXT TAB`    | ++tab++            | :material-check: |
|  `PREV TAB`    | ++shift+tab++      | :material-check: |
|  `OPEN`        | ++return++         |                  |
|  `QUIT`        | ++q++              |                  |
