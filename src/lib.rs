use git2::{Repository, Signature};
use std::error::Error;
use std::path::Path;

pub fn build_commit_types() -> Vec<(&'static str, &'static str)> {
    vec![
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
    ]
}

pub fn format_commit_types(commit_types: Vec<(&str, &str)>) -> Vec<String> {
    // Determine the maximum length of commit type strings for proper alignment
    let max_type_length = commit_types
        .iter()
        .map(|(typ, _)| typ.len())
        .max()
        .unwrap_or(0);

    commit_types
        .iter()
        .map(|(typ, desc)| {
            // Adjust the width to account for proper spacing
            format!("{:<width$} - {}", typ, desc, width = max_type_length + 4)
        })
        .collect()
}

pub fn build_commit_message(
    commit_type: &str,
    scope: &str,
    description: &str,
    body: &str,
    footer: &str,
) -> String {
    let message = format!(
        "{}{}: {}",
        commit_type,
        if scope.is_empty() {
            String::new()
        } else {
            format!("({})", scope)
        },
        description
    );

    let mut full_message = message;

    if !body.is_empty() {
        full_message.push_str(&format!("\n\n{}", body));
    }

    if !footer.is_empty() {
        full_message.push_str(&format!("\n\n{}", footer));
    }

    full_message
}

pub fn perform_commit(repo_path: &Path, full_commit_message: &str) -> Result<(), Box<dyn Error>> {
    let repo = Repository::open(repo_path)?;

    let statuses = repo.statuses(None)?;
    if statuses.is_empty() {
        return Err("Nothing to commit, working directory clean".into());
    }

    let mut index = repo.index()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let config = repo.config()?;
    let author_name = config.get_string("user.name")?;
    let author_email = config.get_string("user.email")?;
    let sig = Signature::now(&author_name, &author_email)?;

    let head = repo.head()?;
    let parent_commit = repo.find_commit(head.target().ok_or("Failed to find HEAD target")?)?;

    repo.commit(
        Some("HEAD"),
        &sig,
        &sig,
        &full_commit_message,
        &tree,
        &[&parent_commit],
    )?;

    Ok(())
}
