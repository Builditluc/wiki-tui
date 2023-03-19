# Api Settings

## Changing the Language

:octicons-milestone-16: Default: `en`

You can change the language of the articles and search by changing the `api.language` setting in you
config file. The default language is english

!!! example "Changing the language to German"
    ```toml
    api.language = "de"
    ```
    Using the langugae name in english is also supported
    ```toml
    api.langugae = "German"
    ```

### Supported Langugaes

| Language name in english | Native Language name | Value |
|--------------------------|----------------------|-------|
| Abkhaz                   | аԥсшәа               | ab    |
| Acehnese                 | Acèh                 | ace   |
| English                  | English              | en    |
| German                   | Deutsch              | de    |

## Changing the site

The options `api.pre_language` and `api.post_language` allow you to change the wiki site to,
theoretically, any Mediawiki based site. Make sure that `api.post_language` points to the api
endpoint of the site, otherwise wiki-tui won't work.

!!! default
    ```toml
    api.pre_language = "https://"
    api.post_language = ".wikipedia.org/w/api.php"
    ```
