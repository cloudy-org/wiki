# Advanced Installations

--8<-- "wip-page.md"

!!! warning
    This is the advanced installation guide. If you're new you'll want the **[Installation Guide](./installation.md)** instead.

## Roseate Slim (aka basic)
Roseate slim is the lean and basic version of Roseate that excludes all the additional features such as the image exif location button and less common file formats.

| Type | Binary Size | Rust Crates |
| ---- | ---- | ----------- |
| Roseate **Basic** | `14.6 MB`  | 427 |
| Roseate **Default** | `22.5 MB` | 461 |

> As of **`0.1.0-alpha.21`** at **22/03/2026** on **Linux**. [^1]

This is a comparison of the two types compiled for **Linux** in **release mode**. The Roseate Linux binary is actually the **biggest binary** so far, hence this is the worse case scenario platform-wise (e.g: the **default** Windows binary is `17 MB`).

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
cargo build --release --no-default-features --features "basic eframe-linux"
```

On Linux you must include the the `eframe-linux` feature to support **Wayland** and **x11**. See this **[section](#display-server-specific-x11-wayland)** on how to avoid compiling both.

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

### Display Server Specific (X11 / Wayland)

!!! note
    The binary size reductions here are nothing to really brag about, unless you're chaining 
    it with other feature flags. This feature flag exists mostly as an example for future flags.

Oh...? what's that, you don't want to compile both Wayland and X11 dependencies?

Well lucky for you! There a feature flags for that too:

```sh title="terminal"
cargo build --release --no-default-features --features "basic eframe-x11-only"
```

> Replace with **`eframe-wayland-only`** to only compile wayland dependencies.
> **Wayland** users actually compile **3** crates less.

[^1]: **Update:** I was just able to reduce it from **476** crates to **461** in **`0.1.0-alpha.21`**.

[goldy_site]: https://devgoldy.xyz
[source_code]: https://github.com/cloudy-org/roseate