**Note: This project is still a WIP and isn't functional**

This application allows you to perform basic Git operations from the
command-line. To get started, make sure you have the latest version of Rust
installed on your system. Then, clone this repository and navigate to the
project directory:

```bash 
git clone https://github.com/my-git-cli.git 
cd my-git-cli
```

To build the application, run:

```bash
cargo build --release
```

This will create a binary executable file in the target/release directory.

To use the application, navigate to the directory of the Git repository you want
to work with and run the my-git-cli command followed by the desired operation
and any necessary arguments. For example, to create a new commit with a message,
run:

```bash
my-git-cli commit -m "My commit message"
```

You can also specify the command and message arguments as --command and
--message, respectively. For example:

```bash
my-git-cli --command commit --message "My commit message"
```

Currently, the available operations are commit, branch, and merge.

Please let me know if you have any feedback or suggestions for improvement!