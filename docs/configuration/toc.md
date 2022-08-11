# Table of Contents Configuration

!!! info
    You can [disable](./features.md#table-of-contents) the table of contents

## Changing the Title

[:octicons-tag-24: 0.5.1][release-0.5.1] ·
:octicons-milestone-16: Default: `default`

The available title options for the table of contents are:

* `default`: uses the table of contents title given by the article (usually "Contents")
* `article`: uses the title of the current article
* `custom`: you can also define a custom title

```toml
settings.toc.title = "default"
```

With a custom title

```toml
settings.toc.title = "custom"
settings.toc.title_custom = "My Custom Title"
```

## Positioning the Table of Contents

[:octicons-tag-24: 0.5.1][release-0.5.1] ·
:octicons-milestone-16: Default: `right`
 
Per default, the table of contents is on the right side of the screen. You can also have the table of contents be on the left side of the screen

```toml
settings.toc.position = "left"
```

## Adjusting the width

You can change the minimal and maximal width of the table of contents

### Minimal Width

[:octicons-tag-24: 0.5.1][release-0.5.1] ·
:octicons-milestone-16: Default: `20`

The minimal width of the table of contents (measured in characters)

```toml
settings.toc.min_width = 20
```

### Maximal Width

[:octicons-tag-24: 0.5.1][release-0.5.1] ·
:octicons-milestone-16: Default: `60`

The maximal width of the table of contents (measured in characters)

```toml
settings.toc.max_width = 60
```

## Disable Scrolling

If don't want the scrolling (and the scrollbars) in your table of contents, you can disable it

### Horizontal Scrolling

[:octicons-tag-24: 0.5.1][release-0.5.1] ·
:octicons-milestone-16: Default: `true`

Will disable horizontal scrolling (and the horizontal scrollbar) if set to `false`

```toml
settings.toc.scroll_x = true
```

### Vertical Scrolling

[:octicons-tag-24: 0.5.1][release-0.5.1] ·
:octicons-milestone-16: Default: `true`

Will disable vertical scrolling (and the vertical scrollbar) if set to `false`

```toml
settings.toc.scroll_y = true
```

## Modify Item generation

[:octicons-tag-24: 0.5.1][release-0.5.1] ·
:octicons-milestone-16: Default: `{NUMBER} {TEXT}`

If you don't like the look of the items in the table of contents, you can modify how these are generated. Available values are:

* `{NUMBER}`: This is the current number of the header (for example: 1, 1.1, 1.2, 2, ...)
* `{TEXT}`: The text (or content) of the header the item represents

```toml
settings.toc.item_format = "{NUMBER} {TEXT}"
```

If you don't like the numbers at the beginning

```toml
settings.toc.item_format = "{TEXT}"
```

Or if you want to have a custom beginning

```toml
settings.toc.item_format = "# {TEXT}"
```

[release-0.5.1]: https://github.com/Builditluc/wiki-tui/releases/tag/v0.5.1