import ctypes
import typing
import logging

from pathlib import Path
from mkdocs.plugins import BasePlugin
from mkdocs.structure.pages import Page
from mkdocs.structure.files import Files
from mkdocs.config import config_options
from mkdocs.config.defaults import MkDocsConfig

__all__ = (
    "AppDocsGenerator",
)

logger = logging.getLogger(__name__)

AppConfigData = typing.TypedDict(
    "AppConfigData",
    {
        "git-url": str,
        "assets-path": str
    }
)

class AppDocsGenerator(BasePlugin):
    config_scheme = (
        ("apps", config_options.Type(dict, default = {})),
    )

    def on_page_markdown(self, markdown: str, page: Page, config: MkDocsConfig, files: Files):
        page_path = Path(page.file.src_path)

        if not "apps" in page_path.parts:
            return markdown

        app_name = page_path.parent.name

        apps = typing.cast(dict[str, AppConfigData], val = self.config["apps"])

        app_config = apps.get(app_name)

        if app_config is not None:
            git_url = app_config.get("git_url")
            assets_path = app_config.get("assets_path")

            app_config_section = find_config_template_and_generate_markdown(
                git_url, assets_path
            )

            markdown = markdown.replace(
                "{wiki-app-config-section}",
                app_config_section
            )

        return markdown

def find_config_template_and_generate_markdown(git_url: str, assets_path: str) -> str:
    # shared_object_path = Path("./target/release/libwiki.so")

    # wiki_shared_lib = ctypes.CDLL(str(shared_object_path.absolute()))

    # wiki_shared_lib.add.argtypes = [ctypes.c_int, ctypes.c_int]
    # wiki_shared_lib.add.restype = ctypes.c_int

    # print(wiki_shared_lib.add(5, 6))

    # TODO: find config.template.toml file, parse it 
    # and generate markdown all on the rust side then 
    # return the string here.
    app_config_section = "*generated config will go here...*"

    return app_config_section