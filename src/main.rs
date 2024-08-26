use git2::{Repository, Signature};
use promkit::preset::query_selector::QuerySelector;
use promkit::{preset::confirm::Confirm, preset::readline::Readline, suggest::Suggest};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commit_types = vec![
        ("feat", "A new feature"),
        ("fix", "A bug fix"),
        ("docs", "Documentation only changes"),
        (
            "style",
            "Changes that do not affect the meaning of the code (white-space, formatting, etc.)",
        ),
        (
            "refactor",
            "A code change that neither fixes a bug nor adds a feature",
        ),
        ("perf", "A code change that improves performance"),
        ("test", "Adding missing tests or correcting existing tests"),
        ("chore", "Other changes that don't modify src or test files"),
        ("ci", "Changes to our CI configuration files and scripts"),
        (
            "build",
            "Changes that affect the build system or external dependencies",
        ),
        ("revert", "Reverts a previous commit"),
    ];

    // Determine the width of the longest commit type
    let max_type_length = commit_types
        .iter()
        .map(|(typ, _)| typ.len())
        .max()
        .unwrap_or(0);

    // Format the commit types with aligned descriptions
    let commit_types_display: Vec<String> = commit_types
        .iter()
        .map(|(typ, desc)| format!("{:<width$} - {}", typ, desc, width = max_type_length))
        .collect();

    let mut p = QuerySelector::new(commit_types_display.clone(), |text, items| -> Vec<String> {
        items
            .iter()
            .filter(|item| item.contains(text))
            .cloned()
            .collect()
    })
    .title("Select the type of change that you're committing:")
    .listbox_lines(10)
    .prompt()?;

    let mut scope_input = Readline::default()
        .title("Denote the scope of this change (optional):")
        .enable_suggest(Suggest::from_iter([
            "app", "core", "ui", "db", "api", "frontend", "backend", "config", "build", "sec",
            "infra", "deps",
        ]))
        .prompt()?;

    let mut description_input = Readline::default()
        .title("Write a SHORT, IMPERATIVE tense description of the change:")
        .prompt()?;
    let mut body_input = Readline::default()
        .title("Provide a LONGER description of the change:")
        .prompt()?;

    // Store the result of p.run() in a variable to avoid borrowing a temporary value
    let selection = p.run()?;
    let selected_type = selection.split_whitespace().next();

    // Handle the Option<&str> safely
    if let Some(commit_type) = selected_type {
        let scope = scope_input.run()?;
        let description = description_input.run()?;
        let body = body_input.run()?;

        // Format the commit message and body properly
        let commit_message = format!(
            "{}{}:{} {}",
            commit_type,
            if scope.is_empty() {
                String::new()
            } else {
                format!("({})", scope)
            },
            if !description.is_empty() { " " } else { "" },
            description
        );

        let full_commit_message = if body.is_empty() {
            commit_message
        } else {
            format!("{}\n\n{}", commit_message, body) // Add two newlines before the body if it's not empty
        };

        /*
        println!("\nCommit Message:");
        println!("---------------");
        println!("{}", full_commit_message);
        */

        // Confirmation before committing using promkit
        let mut confirm_input =
            Confirm::new("Do you want to proceed with this commit?").prompt()?;
        let confirm = confirm_input.run()?;
        if confirm.trim().to_lowercase() == "yes" || confirm.trim().to_lowercase() == "y" {
            // Open the repository
            let repo_path = Path::new("."); // Adjust this path if needed
            let repo = Repository::open(repo_path)?;

            // Get the existing signature from Git config
            let config = repo.config()?;
            let author_name = config.get_string("user.name")?;
            let author_email = config.get_string("user.email")?;
            let sig = Signature::now(&author_name, &author_email)?;

            // Get the index and the working directory
            let mut index = repo.index()?;
            let tree_oid = index.write_tree()?;
            let tree = repo.find_tree(tree_oid)?;

            // Get the HEAD commit
            let head = repo.head()?;
            let parent_commit =
                repo.find_commit(head.target().ok_or("Failed to find HEAD target")?)?;

            // Create the commit
            let commit_oid = repo.commit(
                Some("HEAD"),         // Update the HEAD reference
                &sig,                 // Author signature
                &sig,                 // Committer signature
                &full_commit_message, // Commit message
                &tree,                // Tree to commit
                &[&parent_commit],    // Parent commits
            )?;

            println!("Commit successful! Commit ID: {}", commit_oid);
        } else {
            println!("Commit aborted.");
        }
    }

    Ok(())
}
