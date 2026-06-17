use std::collections::HashSet;

use ammonia::Builder;
use cirrus_config::template::Template;
use cirrus_git_tag::{GitTag, platform::Platform};

use reqwest::Url;
use pulldown_cmark::{Parser, html::push_html};
use pyo3::{exceptions::{PyRuntimeError, PyValueError}, prelude::*};

// NOTE: this is all testing code, I'll need to handle 
// errors and refactor this into something better later.

#[pymodule]
fn wiki(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_function(wrap_pyfunction!(find_config_template_and_generate_markdown, module)?)?;
    Ok(())
}

#[pyfunction]
fn find_config_template_and_generate_markdown<'a>(git_repo_tag: String, version_tag: Option<String>, config_template_path: String) -> PyResult<String> {
    // TODO: handle results

    match parse_git_repo_tag_to_raw_url(git_repo_tag, version_tag) {
        Ok(raw_url) => {
            let template_config_url = Url::parse(&raw_url).unwrap()
                .join(&config_template_path).unwrap();

            let template_config_response = reqwest::blocking::get(template_config_url.clone())
                .unwrap();

            let response_status = template_config_response.status();

            if !response_status.is_success() {
                return Err(
                    // Runtime errors in our wiki plugin will fail doc builds.
                    PyRuntimeError::new_err(
                        format!(
                            "GET response from '{template_config_url}' was not successful! Status Code: {response_status}",
                        )
                    )
                );
            }

            let template_toml_config_string = template_config_response.text().unwrap();

            let mut template = Template::new(template_toml_config_string.as_str());

            if let Err(error) = template.parse_keys() {
                // TODO: use 'error' in place of 'N/A'
                // This Error enum does not implement Display trait atm

                return Err(
                    // Value errors in our wiki plugin will NOT fail the entire doc 
                    // build but just configuration generation of the specific application.
                    PyValueError::new_err(
                        format!(
                            "Failed to parse template config from '{template_config_url}'! Error: N/A",
                        )
                    )
                );
            };

            let mut markdown_source_code = String::new();

            let mut added_section_headings: HashSet<String> = HashSet::new();

            if let Some(template_keys) = template.keys {

                let mut html_sanitizer = Builder::empty();
                html_sanitizer.add_tags(&["p"]); // new lines in the description when parsed to 
                // markdown create new paragraphs so we need to allow the paragraph tag

                for key_path in template.ordered_paths {
                    let template_key = template_keys.get(&key_path);

                    if let Some(template_key) = template_key {
                        let master_key = match key_path.split_once(".") {
                            Some((root_key, _)) => root_key.to_string(),
                            None => key_path.clone(),
                        };

                        let master_section_heading = format!(
                            "\n## {}\n",
                            master_key.split("_")
                                .map(capitalize_first_letter_of_word_map)
                                .collect::<String>()
                        );

                        if added_section_headings.insert(master_key) {
                            markdown_source_code.push_str(&master_section_heading);
                        }

                        markdown_source_code.push_str(
                            &format!(
                                "\n### {}\n",
                                template_key.key
                                    .split("_")
                                    .map(capitalize_first_letter_of_word_map)
                                    .collect::<String>()
                            )
                        );

                        markdown_source_code.push_str(
                            &format!(
                                "\n```toml\n{} = {}\n```\n",
                                key_path,
                                template_key.defined_toml_value
                            )
                        );

                        markdown_source_code.push_str(
                            &format!(
                                "\n**Description:**\n\n{}\n",
                                match &template_key.docstring.description.long {
                                    Some(description) => sanitize_markdown(&html_sanitizer, description),
                                    None => String::from("No description."),
                                }
                            )
                        );
                    }
                }
            }

            Ok(markdown_source_code)
        },
        Err(error) => {
            Err(
                PyRuntimeError::new_err(
                    format!("Failed to parse git repo tag to raw url! Error: {error}")
                )
            )
        },
    }
}

fn capitalize_first_letter_of_word_map(word: &str) -> String {
    let mut chars = word.chars();

    match chars.next() {
        Some(first_char) => format!(
            "{}{} ",
            first_char.to_uppercase().to_string(),
            chars.as_str()
        ),
        None => String::new(),
    }
}

fn sanitize_markdown(html_sanitizer: &Builder, markdown_string: &String) -> String {
    let mut parsed_markdown_html = String::new();

    push_html(
        &mut parsed_markdown_html,
        Parser::new(markdown_string)
    );

    html_sanitizer
        .clean(&parsed_markdown_html)
        .to_string()
}

fn parse_git_repo_tag_to_raw_url(git_repo_tag_string: String, version_tag: Option<String>) -> Result<String, String> {
    // TODO: switch to error struct

    let git_tag = GitTag::parse_string(git_repo_tag_string)
        .map_err(|error| error.to_string())?;

    match git_tag.repo {
        Some(repo) => {
            match git_tag.platform {
                Platform::GitHub => Ok(
                    format!(
                        "https://raw.githubusercontent.com/{}/{}/refs/{}/",
                        git_tag.owner,
                        repo,
                        match version_tag {
                            Some(tag) => format!("/tags/{tag}"),
                            None => String::from("/heads/main"),
                        }
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