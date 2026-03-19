# Advanced Installations

--8<-- "wip-page.md"

!!! warning
    This is the advanced installation guide. If you're new you want the **[Installation Guide](./installation.md)** instead.

## Roseate Slim (aka basic)
Roseate slim is the lean and basic version of Roseate that excludes all the additional features such as the image exif location button and less common file formats.

| Type | Binary Size | Rust Crates |
| ---- | ---- | ----------- |
| Roseate **Basic** | `15 MB`  | 442 |
| Roseate **Default** | `23 MB` | 476 |

> As of **`0.1.0-alpha.20`** on **19/03/2026**.

This is a comparison of the two types compiled for **Linux** in **release mode**. The Roseate Linux binary is actually the **biggest binary** ever, so this is the worse case scenario platform-wise (e.g: the **default** Windows binary is `17 MB`).

> As development proceeds I expect the size to increase gradually, specifically the **Default** type. My goal is to keep our dependency on third-party rust crates as small as possible going forward, I may even replace some libraries with my own but my priority is to get rid of as many as I can. Naturally this means the binary size should also stay pretty consistent or hopefully even shrink. **~ [Goldy][goldy_site]**

### How to compile slim binary.
!!! warning
    This guide is assuming you're on Linux and have **Rust** and **Cargo** installed.

1) Clone the repository and pull git submodules.

```sh title="terminal"
git clone https://github.com/cloudy-org/roseate
cd roseate

git submodule update --init --recursive
```

2) Build Roseate with cargo.

```sh title="terminal"
cargo build --release --no-default-features --features basic
```

3) Install icon, `.desktop` file and binary into your system.

```sh title="terminal"
# Install binary.
sudo cp ./target/release/roseate /usr/bin/

# Install .desktop file and icon.
sudo cp ./app/assets/roseate.desktop /usr/share/applications/
sudo cp ./app/assets/icons/original.png /usr/share/pixmaps/roseate.png

sudo update-desktop-database /usr/share/applications/
```

4) Done!

```sh title="terminal"
roseate
```

[goldy_site]: https://devgoldy.xyz
[source_code]: https://github.com/cloudy-org/roseate