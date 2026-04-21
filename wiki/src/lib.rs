use cirrus_config::v1::template::Template;
use cirrus_git_tag::{GitTag, platform::Platform};

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

    match parse_git_repo_tag_to_raw_url(git_repo_tag) {
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

            // TODO: change 'template.keys' to be ordered
            if let Some(keys) = template.keys {
                for (key_name, template_key) in keys {
                    markdown_source_code.push_str(
                        &format!("\n## `{}`\n", key_name)
                    );

                    markdown_source_code.push_str(
                        &format!(
                            "\n**Default Value:**\n\n```toml\n{}\n```\n",
                            template_key.defined_toml_value
                        )
                    );

                    markdown_source_code.push_str(
                        &format!(
                            "\n**Description:**\n\n{}\n",
                            match template_key.docstring.description.long {
                                Some(description) => description,
                                None => "No description.".into(),
                            }
                        )
                    )
                }
            }

            Ok(markdown_source_code)
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

fn parse_git_repo_tag_to_raw_url(git_repo_tag_string: String) -> Result<String, String> {
    // TODO: switch to error struct

    let git_tag = GitTag::parse_string(git_repo_tag_string)
        .map_err(|error| error.to_string())?;

    match git_tag.repo {
        Some(repo) => {
            match git_tag.platform {
                Platform::GitHub => Ok(
                    format!(
                        "https://raw.githubusercontent.com/{}/{}/refs/heads/main/",
                        git_tag.owner,
                        repo
                    )
                ),
                _ => Err(
                    "Could not find supported git platform tag (only '@ gh' is supported at the moment)!".into()
                ),
            }
        },
        None => Err("No repository was found in the git tag!".into()),
    }
}