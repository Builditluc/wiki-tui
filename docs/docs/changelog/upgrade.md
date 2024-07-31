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
more, read the [docs](../configuration/api.md#changing-the-site)

#### `logging.log_dir`

This option was removed completely because of the fixed path of the log file inside of the cache directory
(can be found when running `wiki-tui --cache-dir`)

### Change of the `-l` CLI argument

This CLI argument was used to override the log level on the fly. Now, it is used to change the language.

> The `--level` flag still exists to change the log level

### Changed paths for the crash log and logfile

The logfile and the crash log have both moved to the cache directory (can be found using `wiki-tui --cache-dir`).
The name of the crash log has also changed, more can be found in the corresponding [doc page](../getting-started/crash.md)
