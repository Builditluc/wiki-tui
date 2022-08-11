# Configuration

The configuration file can be found at the following path

```
$HOME/.config/wiki-tui/config.toml
```

It is written in [toml](https://github.com/toml-lang/toml) and the following pages list all of the available configuration options.
If you are not familiar with time, we suggest you read about the toml specification online.

!!! tip "One Quick Note about TOML"
    TOML lets you group options that are in the same [table](https://toml.io/en/v1.0.0#table). It is suggested you group the options like this. For example, if you have the following options:
    
    ```
    settings.toc.title = ...
    ```

    and 

    ```
    settings.toc.position = ...
    ```

    you can group them like this:

    ```
    [settings.toc]
    title = ...
    position = ...
    ```

    This is the recommended method because it makes your config easier to read for humans

Because wiki-tui is under active development, there will be many more configuration options. If you see any option on this page tagged with `pre-release` that means it hasn't been released yet and will be added on the next release.
