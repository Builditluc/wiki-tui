Show the currently installed version with:
```sh
wiki-tui --version
```

## Upgrade from v0.6.x to v0.7.x

### Changes to `config.toml`

#### `api.base_url`

This option was removed completely and instead replaced by the following three options:

* `api.pre_language`
* `api.post_language`
* `api.language`

> You only have to [modify](../configuration/api.md#changing-the-language) the `api.language` option
> change the language

You most likely never have to change the `api.pre_language` and `api.post_language` options, but
they'll allow changing the site wiki-tui uses (theoretically, every Wikimedia based site works), for
more, read their [docs](../configuration/api.md#changing-the-site)
