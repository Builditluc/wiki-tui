# Contributing

<center>
<b>
Welcome to the Contributing Guide for wiki-tui!
</b>
</center>

We weolcome contributions of any size and kind from the community. Whether you have contributed in
the past or a looking to get started, we are greateful for your help in improving the project.

The following pages will give you an overview of how to contribute to the project. We encourage you
to dive in and start contributing!

## Quicklinks for maintainers

* Merge a pull request [here](./maintainers.md)
* Create an release [here](./maintainers.md)
* Assigning labels [here](./maintainers.md)

## Bug and Crash reports

If you have found a bug or experienced a crash while using wiki-tui, we encourage you to file an
issue. When filing a bug or crash report, please make sure to select and fill out
the correct issue template. If you're not sure what to include in your report, take a look at other
bug or crash reports, as well as issues labeled "good first issue," for guidance. If your still
unsure how to write the report, it's okay to go ahead and submit it anyway. We will be happy to 
help you out and ask for furhter information as needed. Thank you for helping us improve wiki-tui!

You can create a new issue [over here](https://github.com/Builditluc/wiki-tui/issues/new/choose) and
some good first issues can be found
[here](https://github.com/Builditluc/wiki-tui/issues?q=is%3Aopen+label%3A%22good+first+issue%22+sort%3Aupdated-desc)!

## Feature requests and implementations

If you have a feature request or improvement suggestion for wiki-tui, there are a few things to keep
in mind. Smaller features or improvements can often be made directly through a pull request, but
bigger features or improvements should be discussed first in an issue. In general, it's better to
open an issue for feature requests and improvements, even if you just want to ask about the
possibility of a certain feature or improvement. Be sure to always check if other requests have been
made for the same feature or improvement before submitting a new one.

You can create a new issue [over here](https://github.com/Builditluc/wiki-tui/issues/new/choose).
Just select the corresponding issue template and fill it out to your best accords.

## Modify the project

* To modify the project, start by forking and cloning it to your local machine.
* You'll need the following things installed to start working on wiki-tui
    * If you are changing the codebase, you will need to have Rust installed. 
    * If you are changing the documentation, you will need to have Docker installed.
    * Also be sure to have a terminal that supports `true-colors` when changing the codebase
* Create a new branch for your changes, and name it something that describes what it changes. 
  There is no strict naming convention, but it should be clear and descriptive. 
* Make your changes 
* Before committing, it's important to check that your changes are correct. 
    * If you are modifying the documentation, you can do this by executing the `mkdocs-serve.sh` 
      file and opening the local documentation site in your browser. 
    * If you are modifying the codebase, be sure to run `cargo clippy`, `cargo fmt`, and `cargo 
      test` 

!!! example inline end "Example commit messages"
    * `fix: fix crash on empty query`
    * `fix: fix external links not being recognized`
    * `docs: change github workflow badge routes`
    * `chore: update contributors badge style`

* Create a commit for each change you have made. <br/><br/> There is no strict 
  convention for commit messages, but it's helpful to begin with the type of change (e.g. `fix,` 
  `feature,` `improvement,` etc.) and then describe the change itself. 

!!! important
    Please keep sure that each commit is focused on a single change, as this will make it easier to 
    figure out what commit caused any issues that may arise later. 

* Push your changes to your fork.
* Create a pull request to merge your changes back into the main project.
    * Name your pull request according to what it changes, and include a clear and detailed 
      description.
    * If your pull request is related to an issue, be sure to include a link to the issue in the
      description. 
    * Include a list of the changes this pull request makes. This list will be used as the 
      description for the merge commit, so it should be clear and informative. If you have written 
      informative commit messages, you can often use them as the basis for this list.
