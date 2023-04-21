# Api Settings

## Changing the Language

[:octicons-tag-24: 0.1.0][release-0.1.0] ·
:octicons-milestone-16: Default: `https://en.wikipedia.org/`

wiki-tui uses the base url to make calls to the wikipedia api. If you want to change the language of the articles and search results, you can change it here in your base url. 

For example if you want to change to the german version of wikipedia, you can just change the base url

```toml
api.base_url = "https://de.wikipedia.org/"
```

[release-0.1.0]: https://github.com/Builditluc/wiki-tui/releases/tag/v0.1
