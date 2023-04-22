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
