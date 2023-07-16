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
  convention for commit messages, but it's helpful, to begin with the type of change (e.g. `fix,` 
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
