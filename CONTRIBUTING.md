# Contributing to wiki-tui
First of all, thank you for taking the time to contribute!

## How can I
### Report a bug?
You can report a bug by opening an Issue at https://github.com/Builditluc/wiki-tui/issues.
> Note: Before filing a bug report, please check if there were any previous reports describing your bug.

#### What should be included in the Bug Report?

* Describe the exact steps which reproduce the problem in as many details as possible
* If you're experiencing a crash, please provide the crash report
* Include the logfile 
* The method you used to install wiki-tui (cargo, nix, compiling, etc.)
* The version of wiki-tui you're currently running

#### What will happen once you've created the Issue?

* The team will [label the issue](#label-issues)
* A maintainer will then try to reproduce the issue with your provided steps
* If the issue could be reproduced, it will be marked `needs-fix` and the issue will be left to be [implemented by someone](#contribute-code).

### Suggest an Enhancement?
If wiki-tui doesn't do something you need or want it to do, open an Issue at https://github.com/Builditluc/wiki-tui/issues.
> Note: Before suggesting an enhancement, please check if there were any previous suggestions describing the same thing.

#### What should be included in the Enhancement Request

* Provide as much context as you can about what you're running into.
* Please try and be clear about why existing features and alternatives would not work for you.

#### What will happen once you've created the Issue?

* The team will [label the issue](#label-issues)
* The team will then evaluate the feature request, possibly asking you more questions. If the issue is closed, the team will convey their reasoning.
* If the request is accepted, it will be marked with `enhancement-accepted`, which can then be done by either a team member or by anyoune who wants to [contribute code](#contribute-code).

### Setup the Project
This project uses GitHub Pull Requests to manage contributions.
To run the project locally:

* Install [rust](https://www.rust-lang.org/tools/install)
* Fork the project
* `cd path/to/your/clone`
* `cargo build`

Then, branch off from the `experimental` branch:

`git checkout -b myfeature experimental`

And you're good to go!

### Contribute Code
Code contributions of just about any size are acceptable!
To contribute code:

* [Set up](#setup-the-project) the project
* Make any necessary changes to the source code
* Format your code with `cargo fmt` and `cargo clippy`
* Write clear, concise commit messages(s) using [commit style guide](#git-commit-messages)
* Open a new pull request with your changes (merge with the experimental
  branch)
* If your pull request is connected to an open issue, add a line in your PR's description that says `Fixes: #1234`, where `#1234` is the number of the issue you're fixing.
* If your pull request was merged, feel free to [add yourself as a contributor](https://allcontributors.org/docs/en/bot/usage)

## Style Guide
### Git Commit Messages
wiki-tui uses the commit messages style from the [auto-generate-changelog](https://github.com/BobAnkh/auto-generate-changelog/blob/master/CONTRIBUTING.md#commit-message-convention) repo.
Each commit should contain relatively independent change (that is, a hodgepodge of multiple types of modifications is not allowed to be submitted in one commit), and the specific changes need to be clarified in the message

The commit message conventions of this project mainly refers to the most widely used [AngularJS Git Commit Message Conventions](https://docs.google.com/document/d/1QrDFcIiPjSLDn3EL15IJygNPiHORgU1_OOAqWjiDU5Y/edit#heading=h.uyo6cb12dt6w)

Here is the message format:

```
<type>(<scope>): <subject>
// BLANK LINE
<body>
// BLANK LINE
<footer>
```
The `<header>` section(the first line) is mandatory for any project. The `<body>` section and `<footer>` section are optional according to the actual situation

A blank line is required between sections

Also, the `<header>` section(only contains one line) cannot be longer than 50 characters and any line of the `<body>` section cannot be longer than 72 characters

This allows the commit message to be easier to read on GitHub as well as in various git tools.

#### About `<header>` Section

The `<header>` section only contains one line and three fields(`<type>`, `<scope>` and `<subject>`) need to meet the requirements:

The `type` field mainly explains the type of the commit. Only the following 9 types are allowed to be used in `AngularJS Git Commit Message Conventions`:

* feat: A new feature
* fix: A bug fix
* docs: Documentation only changes
* style: Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
* refactor: A code change that neither fixes a bug nor adds a feature
* perf: A code change that improves performance
* test: Adding missing or correcting existing tests
* chore: Changes to the build process or auxiliary tools and libraries such as documentation generation
* revert: If the commit reverts a previous commit, it should be followed by the `<header>`of the reverted commit and in the body it should say: `This reverts commit <hash>.`, where the hash is the SHA of the commit being reverted

If a commit is related to multiple `<type>`, use `feat` and `fix` in priority, followed by the remaining seven types specified in `AngularJS Git Commit Message Conventions`, and the remaining two are for special needs

The `<scope>` section mainly describes the influence scope of this commit. It should be the name of the module affected.

The following is the list of supported scopes:
* readme: changes to the readme
* contributing: changes to CONTRIBUTING.md
* cargo: changes to Cargo.toml
* ui: changes to the ui
* wiki: changes in the wiki module 
* config: changes to the config
* logging: changes to the logging
* error: changes to error-handling
* `*`

The `<subject>` section mainly summarizes the purpose and changes of this commit. It should begin with verb and use the imperative, present tense. The first letter should be lowercase and have no dot(.) at the end

#### About `<body>` Section

The `<body>` section is the text section, which contains the detailed description of this commit. It should use the imperative, present tense

This section can be bypassed if the `<header>` section is enough to summarize the entire change of this commit

It is recommended to use the dashes(-) to create an unordered list, and it should explain what problem this commit solves, how to solve it, and whether other changes have been introduced (such as necessary document updates, etc.)

#### About `<footer>` Section

The `<footer>` section is bypassed except 2 situations:

One is breaking change, that is, the current version is not compatible with the previous version. It should start with the word `BREAKING CHANGE:` with a space or two newlines. The rest of the breaking change block is then the description of the change, justification and migration notes.

The other is to reference GitHub issues that this commit closes. Use format `Closes #123, #456` to close one or more issues

#### Commit Message Examples

Here are some examples of commit message:

> For example, if a new feature is to add a option for round contributor's avatar, the commit message can be written as:
```text
feat(contributor): add a option for round avatar
- add a option to choose the avatar in circle or in square
- add new template in the python script to handle it
- update usage and example in README.md
Closes #123
```

> If a new documentation of linux command ls is added, the commit message can be written as:
```text
docs(command): add linux command ls
- add basic usage format of command ls
- add arguments of command ls
- add considerations of command ls
- plan to add more typical examples in future
- plan to add descriptions in the future
```

> If it fixes a typo found in the documentation ls.md, the commit message can be written as:
```text
docs(ls.md): fix a typo
- change `-` to `--`
Closes #456
```

## Label Issues
Label | Apply When | Notes
--- | --- | ---
`bug` | Cases where the code (or documentations) is behaving in a way it wasn't intended to. | If something is happening that surprises the user but does not go against the way the code is designed, it should use the enhancement label.
`critical` | Added to `bug` issues if the problem described makes the code completely unusable in a common situation. |
`documentation` | Added to issues or pull requests that affect any of the documentation for the project. | Can be combined with other labels, such as `bug` or `enhancement`
`duplicate` | Added to issues or PRs that refer to the exact same issue as another one that's been previously labeled. | Duplicate issues should be marked and closed right away, with a message referencing the issue it's a duplicate of (with `#123`)
`enhancement` | Added to feature requests, PRs, or documentation issues that are purely additive: the code or docs currently work as expected, but a change is being requested or suggested. | 
`wontfix` | Labelers may apply this label to issues that clearly have nothing at all to do with the project or are otherwise entirely outside of its scope/sphere of influence. Committers may apply this label and close an issue or PR if they decide to pass on an otherwise relevant issue. | The issue or PR should be closed as soon as the label is applied, and a clear explanation provided of why the label was used. Contributors are free to contest the labeling, but the decision ultimately falls on committers as to whether to accept something or not.
