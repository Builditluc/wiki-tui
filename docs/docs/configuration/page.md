# Page Configuration

In this page you can find all of the configuration options for the page. In the configuration file,
this describes the `[page]` table.

## Table of Contents Configuration

### Disabling the Table of Contents

[:octicons-tag-24: 0.5.0][release-0.5.0] · :octicons-milestone-16: Default: `true`

By settings this to false, the table of contents won't be displayed

```toml
page.toc.enabled = false
```

### Changing the Title

[:octicons-tag-24: 0.5.1][release-0.5.1] ·
:octicons-milestone-16: Default: `default`

The available title options for the table of contents are:

* `Default`: uses the title `Contents`
* `Article`: uses the title of the current article
* `Custom`: you can also define a custom title

```toml
page.toc.title = "Default"
```

With a custom title

```toml
page.toc.title = { Custom = "My Custom Title" }
```

### Positioning the Table of Contents

[:octicons-tag-24: 0.5.1][release-0.5.1] ·
:octicons-milestone-16: Default: `Right`
 
Per default, the table of contents is on the right side of the screen. You can also have the table 
of contents be on the left side of the screen

```toml
page.toc.position = "Left"
```

### Adjusting the width

[:octicons-tag-24: 0.9.0][release-0.9.0] · :octicons-milestone-16: Default `20`

You can change the percentage of the space the table of contents will occupy. The values are in
percent (only whole numbers between 0 and 100 are allowed)

```toml
page.toc.width_percentage = 20
```

### Disable Scrolling

[:octicons-tag-24: 0.9.0][release-0.9.0] ·
:octicons-milestone-16: Default: `true`

Will disable scrolling inside of the table of contents if set to `false`

```toml
page.toc.enable_scrolling = true
```

### Modify Item generation

[:octicons-tag-24: 0.5.1][release-0.5.1] ·
:octicons-milestone-16: Default: `{NUMBER} {TEXT}`

If you don't like the look of the items in the table of contents, you can modify how these are generated. Available values are:

* `{NUMBER}`: This is the current number of the header (for example: 1, 1.1, 1.2, 2, ...)
* `{TEXT}`: The text (or content) of the header the item represents

```toml
page.toc.item_format = "{NUMBER} {TEXT}"
```

If you don't like the numbers at the beginning

```toml
page.toc.item_format = "{TEXT}"
```

Or if you want to have a custom beginning

```toml
page.toc.item_format = "# {TEXT}"
```

[release-0.9.0]: https://github.com/Builditluc/wiki-tui/releases/tag/v0.9
[release-0.5.1]: https://github.com/Builditluc/wiki-tui/releases/tag/v0.5.1
[release-0.5.0]: https://github.com/Builditluc/wiki-tui/releases/tag/v0.5.0
