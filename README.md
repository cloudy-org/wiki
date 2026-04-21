<div align="center">

  # 📖 Wiki

  <sub>Our static wiki site.</sub>

</div>

## Developer Note
We use [mkdocs](https://www.mkdocs.org/getting-started/) for this site. To serve the site for development you'll need **Python** and project's pip dependencies installed.

Git clone the repo, create a virtual environment, pip install the dependencies and then serve the site for development:
```sh
git clone https://github.com/cloudy-org/wiki
cd wiki

python -m venv .venv
source .venv/bin/activate

pip install . -U

mkdocs serve
```

### With uv
```sh
git clone https://github.com/cloudy-org/wiki
cd wiki

# UV grabs all the dependencies and does all the venv stuff for us automatically.
uv run mkdocs serve
```

## `libwiki.so`
The application configuration page requires the Rust wiki crate to be compiled before it can generate:

```sh
# Pull cirrus toolkit
git submodule update --init --recursive

cargo build --release
```