extern crate git2;

use clap::{App, Arg};
use git2::{Commit, Repository};

/*
This function creates a new commit in the given repository with the given message.

It does this by:
  - getting the signature for the current user
  - writing the current index (a list of file changes to be committed) to a tree object
  - creating a commit object with the tree, signature, and message
  - returning the created commit object
*/
fn commit<'a>(repo: &'a Repository, message: &str) -> Result<Commit<'a>, git2::Error> {
    // Gets the signature for the current user.
    let signature = repo.signature()?;
    // Writes the current index to a tree object.
    let tree_id = repo.index()?.write_tree()?;
    // Gets the tree object
    let tree = repo.find_tree(tree_id)?;
    // Creates a commit object with the tree, signature, and message.
    let oid = repo.commit(Some("HEAD"), &signature, &signature, message, &tree, &[])?;
    // Returns the created commit object.
    repo.find_commit(oid)
}

fn main() -> Result<(), git2::Error> {
    // Parse the repository path from the command-line arguments.
    let repo_path = std::env::args().nth(1).expect("missing repo path argument");

    // Create a new instance of the `clap` command line argument parser.
    let app = App::new("my-git-cli");

    // Create a command argument that is required and has the possible values "commit", "branch", and "merge".
    let command_arg = Arg::with_name("command")
        .required(true)
        .index(1)
        .possible_values(&["commit", "branch", "merge"]);

    // Create a message option that is optional and has a short and long version (e.g., "-m" and "--message").
    let message_opt = Arg::with_name("message")
        .long("message")
        .short('m')
        .takes_value(true);

    // Add the command and message options to the `clap` app.
    let app = app.arg(command_arg).arg(message_opt);

    // Parse the command-line arguments using `clap`.
    let matches = app.get_matches();

    // Get the value of the command argument.
    let command = matches.value_of("command").unwrap();

    // Get the value of the message option, or a default value if it wasn't provided.
    let message = matches.value_of("message").unwrap_or("No message");

    println!("Command: {}", command);
    println!("Message: {}", message);

    // If the command is "commit", create a new commit in the repository with the given message.
    if command == "commit" {
        let repo = Repository::open(&repo_path)?;
        let commit = commit(&repo, message)?;
        println!("Created commit {}", commit.id());
    }

    Ok(())
}
