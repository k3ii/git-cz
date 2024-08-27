use git_cz::{build_commit_message, build_commit_types, format_commit_types, perform_commit};
use promkit::preset::query_selector::QuerySelector;
use promkit::{preset::confirm::Confirm, preset::readline::Readline, suggest::Suggest};
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let commit_types = build_commit_types();
    let commit_types_display = format_commit_types(commit_types);

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

    let selection = p.run()?;
    let selected_type = selection.split_whitespace().next();

    if let Some(commit_type) = selected_type {
        let scope = scope_input.run()?;
        let description = description_input.run()?;
        let body = body_input.run()?;

        let full_commit_message = build_commit_message(&commit_type, &scope, &description, &body);

        let mut confirm_input =
            Confirm::new("Do you want to proceed with this commit?").prompt()?;
        let confirm = confirm_input.run()?;
        if confirm.trim().to_lowercase() == "yes" || confirm.trim().to_lowercase() == "y" {
            perform_commit(Path::new("."), &full_commit_message)?;
            println!("Commit successful!");
        } else {
            println!("Commit aborted.");
        }
    }

    Ok(())
}
