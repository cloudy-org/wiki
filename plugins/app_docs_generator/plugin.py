import typing

import logging
from pathlib import Path
from mkdocs.plugins import BasePlugin
from mkdocs.config import config_options
from mkdocs.config.defaults import MkDocsConfig
from importlib.util import spec_from_file_location, module_from_spec

__all__ = (
    "AppDocsGenerator",
)

logger = logging.getLogger(__name__)

AppConfigData = typing.TypedDict(
    "AppConfigData",
    {
        "git-tag": str,
        "assets-path": str
    }
)

class AppDocsGenerator(BasePlugin):
    config_scheme = (
        ("apps", config_options.Type(dict, default = {})),
    )

    def on_config(self, config: MkDocsConfig):
        docs_path = Path(config["docs_dir"])

        apps = typing.cast(dict[str, AppConfigData], val = self.config["apps"])

        for app_name, app_config in apps.items():
            git_tag = app_config.get("git-tag")
            assets_path = app_config.get("assets-path")

            if git_tag is None or assets_path is None:
                raise KeyError(
                    "Both 'git-tag' and 'assets-path' keys must be set for 'wiki-app-docs-gen'!"
                )

            app_config_markdown_content = find_config_template_and_generate_markdown(
                git_tag, assets_path
            )

            generated_snippets_path = docs_path.parent.joinpath("snippets", "generated")

            if not generated_snippets_path.exists():
                generated_snippets_path.mkdir()

            app_config_docs_markdown_path = generated_snippets_path.joinpath(
                f"{app_name}-config-gen.md"
            )

            with open(app_config_docs_markdown_path, "w") as file:
                file.write(app_config_markdown_content)

def find_config_template_and_generate_markdown(git_repo_tag: str, assets_path: str) -> str:
    shared_object_path = Path("./target/release/libwiki.so")

    shared_lib_spec = spec_from_file_location("wiki", shared_object_path)
    shared_lib_module = module_from_spec(shared_lib_spec)

    # sys.modules["wiki"] = shared_lib_module
    shared_lib_spec.loader.exec_module(shared_lib_module)

    # TODO: find config.template.toml file, parse it 
    # and generate markdown all on the rust side then 
    # return the string here.
    app_config_section = shared_lib_module.find_config_template_and_generate_markdown(
        git_repo_tag,
        assets_path
    )

    return app_config_section