use cirrus_config::v1::template::Template;

use reqwest::Url;
use pyo3::{exceptions::PyRuntimeError, prelude::*};

// NOTE: this is all testing code, I'll need to handle 
// errors and refactor this into something better later.

#[pymodule]
fn wiki(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(find_config_template_and_generate_markdown, module)?)?;
    Ok(())
}

#[pyfunction]
fn find_config_template_and_generate_markdown<'a>(git_repo_tag: String, assets_path_string: String) -> PyResult<String> {
    // TODO: handle results

    match parse_git_repo_tag_to_raw_url(git_repo_tag.clone()) {
        Ok(raw_url) => {
            let template_config_url = Url::parse(&raw_url).unwrap()
                .join(&assets_path_string).unwrap()
                .join("config.template.toml").unwrap();

            let template_toml_config_string = reqwest::blocking::get(template_config_url)
                .unwrap()
                .text()
                .unwrap();

            let mut template = Template::new(template_toml_config_string.as_str());
            template.parse_keys().unwrap();

            let mut markdown_source_code = String::new();

            markdown_source_code.push_str("*generated config will go here... meow*");

            Ok(
                markdown_source_code
            )
        },
        Err(error) => {
            Err(
                // probably should be a different error type or a custom one
                PyRuntimeError::new_err(
                    format!("Failed to parse git repo tag to raw url!\n   Error: {error}")
                )
            )
        },
    }
}

fn parse_git_repo_tag_to_raw_url(git_repo_tag: String) -> Result<String, String> {
    // TODO: switch to error struct

    let repo_and_platform: Vec<&str> = git_repo_tag.split("@")
        .map(|tag_part| tag_part.trim())
        .collect();

    let mut repo_and_platform_iter = repo_and_platform.into_iter();

    match repo_and_platform_iter.next() {
        Some(user_and_repo) => {
            let platform = repo_and_platform_iter.next();

            match platform {
                Some("gh") => {
                    Ok(
                        format!(
                            "https://raw.githubusercontent.com/{}/refs/heads/main/", user_and_repo
                        )
                    )
                },
                _ => Err(
                    "Could not find supported git platform tag (only '@ gh' is supported at the moment)!".into()
                ),
            }
        },
        None => Err(
            format!("Failed to parse git repo tag for user and repo. Git repo tag: {git_repo_tag}")
        ),
    }
}