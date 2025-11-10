# Configuration

!!! info "Note from the Dev"
    I'm planning on overhauling the structure of the configuration. This
    includes but is not limited to:

    - Changing the names of various settings and parameters
    - Organizing the configuration into a better thought out structure

    This means that all of this is subject to change, however, it'll take a
    while for this to be "done".

    > We are also still in early development (pre 1.0), so expect breaking
    > changes to occur every now and then. Always read the docs!

The path to the configuration file can be found by running this command

```
wiki-tui --config-path
```

It's usually this path:

```
$HOME/.config/wiki-tui/config.toml
```

Configuration is separated into the actual configuration and theming. The theme configuration
options can be found [here](./theme.md).

It is written in [toml](https://github.com/toml-lang/toml) and the following pages list all of the available configuration options.
If you are not familiar with toml, I suggest you read about the toml specification online.

!!! tip "One Quick Note about TOML"
    TOML lets you group options that are in the same [table](https://toml.io/en/v1.0.0#table). It is suggested you group the options like this. For example, if you have the following options:
    
    ```
    page.toc.title = ...
    ```

    and 

    ```
    page.toc.position = ...
    ```

    you can group them like this:

    ```
    [page.toc]
    title = ...
    position = ...
    ```

    This is the recommended method because it makes your config easier to read, but it has no effect
    on the application
