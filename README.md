# git-cz

`git-cz` is a simple commitizen tool written in rust.

`git-cz` helps you create [conventional commits](https://www.conventionalcommits.org/en/v1.0.0/) with ease. 
It prompts you to select the type of commit, enter a scope (optional), and provide a summary and detailed description. 

![demo](./assets/demo.gif)

## Features

- Prompts for commit type, scope, summary, and detailed description.


## Installation

```bash
cargo install git-commitizen
```
> [!NOTE]
> Although the package is called `git-commitizen` on crates.io, the installed binary is named `git-cz`:

Check [releases](https://github.com/k3ii/git-cz/releases) for pre-built binaries or compiled binary for your platform.

## Conventional Commits

The [Conventional Commits](https://www.conventionalcommits.org/) specification is a convention for writing consistent commit messages. It helps in automating the release process, generating changelogs, and making it easier to understand the history of a project.

### Format

A conventional commit message has the following format:


```md
<type>(<scope>): <summary>

<body>

<footer>
```

- `type`: The type of change being committed, e.g., feat, fix, docs, etc.
- `scope` (optional): The part of the codebase that is being affected.
- `summary`: A brief summary of the change.
- `body` (optional): A detailed description of the change.
- `footer` (optional): A footer section that can contain additional information about issues fixed or closed. 

Benefits

- Automated Changelog: By following a consistent commit message format, tools can automatically generate changelogs.
- Better Collaboration: It makes it easier for teams to understand the nature of changes in the codebase.
- Semantic Versioning: Helps in determining the version of the project automatically based on the types of commits.

## How Git Picks Up git-cz

Git has a feature called "Git command aliases" that allows it to treat any executable file named `git-<command>` in your `PATH` as a Git subcommand. This is how `git-cz` works:

1. When you run `git cz`, Git looks for an executable file named `git-cz` in the directories listed in your PATH.
2. If it finds such a file (which is our script), Git executes it as if it were a built-in Git command.
3. This mechanism allows you to create custom Git commands without modifying Git itself.

By naming our script `git-cz` and placing it in a directory in the PATH, we're effectively creating a new Git command `cz` that can be invoked as `git cz`.
This approach is flexible and allows for easy installation and use across different projects without requiring project-specific configuration.

## See also

- [streamich/git-cz](https://github.com/streamich/git-cz)
- [Zhengqbbb/cz-git](https://github.com/Zhengqbbb/cz-git)

## Contributing

Feel free to improve the script to add your own commit types or make any other enhancements you think are useful. Contributions are welcome!

If you would like to contribute:

1. Fork the repository.
2. Create a new branch for your feature or fix.
3. Make your changes.
4. Submit a pull request.

Your contributions can help make this tool even more useful for everyone. Thank you for considering contributing!
