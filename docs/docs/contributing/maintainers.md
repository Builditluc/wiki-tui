# Maintainer Guides

## Merging a pull request

* Be sure that all CI checks are passing before merging the pull request.
* Review the code, ask for any necessary changes, and approve the pull request when it is ready.
* Click the `Squash and merge` button to merge the pull request.
* Add the list of changes specified in the pull request description (if not already included) to the 
  merge commit description. If the list of changes is not available, ask the author for it.
* Click `Confirm` to complete the merge.
* If the user has not yet been added as a contributor, use the following command to add them: 
  `@all-contributors please add @<username> for <contributions>` and then merge the created pull 
  request. See the [Emoji Key (Contribution Types Reference)](https://allcontributors.org/docs/en/emoji-key) 
  for a list of valid `contribution` types.

!!! tip
    Your request to the bot doesn't need to be perfect. The bot will use basic Natural Language 
    Parsing to determine your intent. For example, this will work too:

    ```
    Jane you are crushing it in the documentation and your infrastructure work has been great too. 
    Let's add @jane.doe23 for her contributions. cc @all-contributors
    ```

    The bot will work best in parsing your comment correctly if you precede the contributor's 
    username with @ as shown above. Otherwise, the bot may not correctly identify the user.

## Creating an release

To create a new release of wiki-tui, follow these steps:

* Make sure you have a clean working directory by running `git status` and ensuring that there are 
  no uncommitted changes. If there are uncommitted changes, commit them or stash them using `git 
  stash`.
* Check the codebase 
    * For any formatting issues or warnings using the following commands:
        * `cargo clippy`
        * `cargo fmt`
* Run `cargo build` to build the project. 
* Bump the version number in `Cargo.toml` to the next release version.
* Run `cargo build` again to build the project with the updated version number. Don't forget to `git 
  add Cargo.lock` so that the build stays reproducible.
* Update the changelog using `auto changelog --from <old-version> --to <version>`, where
  <old-version> is the last version released and <version> is the new release version.
* Modify the Changelog:
    * Remove any changes from bots
    * Check that everything is correct
* Amend the `Cargo.toml` and `Cargo.lock` changes to the commit `auto` generated and rename the
  commit to `bump to <version>`, where <version> is the new release version using `git commit
  --amend`
  For example `bump to v0.6.2`
* Create a tag using `git tag <version>`, where <version> is the new release version.
* Push the commit using `git push`.
* Push the tag with `git push --tags`.
* Create a new release manually and copy the changelog section of the new
  release into the body field
* Publish the release to crates.io using `cargo publish`
* Wait for the continuous integration (CI) process to complete building the precompiled binaries.
* Finally, run `cargo install wiki-tui` to install the new release onto your local machine, and 
  verify that it is working correctly.

## Nix Development Shell for the documentation website

When using nix, the following command can be used to quickly enter a shell with all the necessary
requirements for developing the site (running a test version locally, publishing a new version, etc.)

```sh
nix-shell -p \
    python312Packages.mkdocs \
    python312Packages.mike \
    python312Packages.mkdocs-material \
    python312Packages.mkdocs-git-revision-date-localized-plugin \
    python312Packages.pillow \
    python312Packages.cairosvg
```

## Versioning

Versioning is done using [mike](https://github.com/jimporter/mike), a tool which lets us manage
multiple versions of our mkdocs documentation site.

When deploying a new version, if it is a new minor version (eg. 0.8->0.9), we deploy a new version 

```sh 
mike deploy --update-aliases 0.9 latest
```

If however, we want to deploy a new patch (eg. 0.8.0->0.8.1), we delete the old patch and deploy the
new one
```sh
mike delete 0.8.0
mike deploy --update-aliases 0.8.1 latest
```

To verify the changes, we can run `mike list`, and when everything is correct, we can publish the
site by pushing the local `gh-pages` branch.

### Development Versions

After updating the documentation site, we create / update the `develop` version of the site, which
gets deleted after publishing a new version.
