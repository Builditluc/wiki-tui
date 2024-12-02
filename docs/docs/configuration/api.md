# Api Settings

## Search Settings

### Search results limit
[:octicons-tag-24: 0.9.0][release-0.9.0] · :octicons-milestone-16: Default `10`

You can configure how many total pages to return per search. The value must be between 1 and 500

```toml
api.search_limit = 10
```

### Changing the QiProfile
[:octicons-tag-24: 0.9.0][release-0.9.0] · :octicons-milestone-16: Default `engineautoselect`

Select the query independent profile to use which affects the ranking algorithm of the search.
Available profiles are

| Name                | Description                                                                              | Config Name           |
|---------------------|------------------------------------------------------------------------------------------|-----------------------|
| Classic             | Ranking based on the number of incoming links, some templates, page language and recency | `classic`             |
| ClassicNoBoostLinks | Ranking based on some templates, page language and recency when activated on the wiki    | `classicnoboostlinks` |
| WSumIncLinks        | Weighted sum based on incoming links                                                     | `wsuminclinks`        |
| WSumIncLinksPV      | Weighted sum based on incoming links and weekly pageviews                                | `wsuminclinkspv`      |
| PopularIncLinksPV   | Ranking based primarily on page views                                                    | `popularinclinkspv`   |
| PopularIncLinks     | Ranking based primarily on incoming links                                                | `popularinclinks`     |
| EngineAutoselect    | Let the search engine decide on the best profile to use                                  | `engineautoselect`    |

```toml
api.search_qiprofile = "engineautoselect"
```

### Changing the search type
[:octicons-tag-24: 0.9.0][release-0.9.0] · :octicons-milestone-16: Default `text`

There are multiple types of search that can be performed by wikipedia. Available settings are

- *`nearmatch`* Search just by a match
- *`text`* Search the content of the page
- *`title`* Search the title of the page

```
api.search_type = "text"
```

### Enabling query rewrites
[:octicons-tag-24: 0.9.0][release-0.9.0] · :octicons-milestone-16: Default `false`

Enable interal query rewriting. Wikipedia can rewrite the query into another which is thought to
provide better results by, for instance, correcting spelling errors

```toml
api.search_rewrites = true
```

### Configuring the sort order
[:octicons-tag-24: 0.9.0][release-0.9.0] · :octicons-milestone-16: Default `relevance`

Set the sort order of returned results. Available sort orders are:

- *`createtimestampascending`*: Sort the results by their creation date in ascending order
- *`createtimestampdescending`*: See above, but in descending order
- *`incominglinksascending`*: Sort the results by their amount of pages linking to it in ascending order
- *`incominglinksdescending`*: See above, but in descending order
- *`justmatch`*: Sort the results only by their match to the query
- *`lasteditascending`*: Sort the results by the time of their last edit in ascending order
- *`lasteditdescending`*: See above, but in descending order
- *`nosort`*: Don't sort the search results
- *`random`*: Arrange the results in a random order
- *`relevance`*: Sort the results by relevance

```toml
api.search_sort_order = "relevance"
```

## Page Settings

### Resolving redirects
[:octicons-tag-24: 0.9.0][release-0.9.0] · :octicons-milestone-16: Default `false`

Whether to resolve page redirects directly

```toml
api.page_redirects = false
```

## Changing the Language

:octicons-milestone-16: Default: `en`

You can change the language of the articles and search by changing the `api.language` setting in you
config file. The default language is English. 

!!! example "Changing the language to German"
    ```toml
    api.language = "de"
    ```

    Using the language name in english aswell as the local language is also supported
    
    ```toml
    api.language = "german"
    ```

    ```toml
    api.language = "deutsch"
    ```
   

### Hiding the language changed popup

:octicons-milestone-16: Default: `true`

You can hide the 'Changed language to ...' popup by disabling the following setting

```toml
ui.popup_search_language_changed = false
```

### Hiding the article language changed popup

:octicons-milestone-16: Default: `true`

You can hide the 'Changed the language of your article to ...' popup by
disabling the following setting

```toml
ui.popup_page_language_changed = false
```

### Supported Languages

A list of all languages can be found here:

[https://en.wikipedia.org/wiki/List_of_Wikipedias](https://en.wikipedia.org/wiki/List_of_Wikipedias#Basic_list)

!!! important "About the support of languages"
    Some versions of Wikipedia have **custom formatting** different from the main wikipedia site. That means changing the language
    to something other than `English` can lead to incorrect formatting or missing data. A fix is planned but will take time

## Changing the site

The options `api.pre_language` and `api.post_language` allow you to change the wiki site to,
theoretically, any Mediawiki based site. Make sure that `api.post_language` points to the api
endpoint of the site, otherwise wiki-tui won't work.

!!! default
    ```toml
    api.pre_language = "https://"
    api.post_language = ".wikipedia.org/w/api.php"
    ```

[release-0.9.0]: https://github.com/Builditluc/wiki-tui/releases/tag/v0.9
