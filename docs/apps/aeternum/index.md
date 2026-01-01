# 🔮 Aeternum
![showcase image](./showcase.png)

> Aeternum is a **free** and **open-source**, image upscaler powered by [ncnn](https://github.com/Tencent/ncnn) via [upscayl-ncnn](https://github.com/upscayl/upscayl-ncnn).

Being a cloudy-org application, it naturally follows the [Cloudy Philosophy](https://github.com/cloudy-org/.github/blob/main/philosophy.md).

It's designed to be **lightweight**, **privacy-respecting** and **simple**.

## Background
Aeternum was actually started by [me](https://ananas.moe) because i wanted to broaden the [cloudy-org](https://github.com/cloudy-org) catalogue and also a way of learning [rust](https://rust-lang.org/) aswell as [egui](https://github.com/emilk/egui).

## Install
See [install.md](./install.md) for the installation guide.

## How to use?
!!! info
    At the moment, MacOS support is unknown, [me](https://ananas.moe) aswell as [goldy](https://devgoldy.xyz) don't own anything that runs MacOS. If you own a device with MacOS, please try running it and let us know via an [issue](https://github.com/cloudy-org/aeternum/issues/new) how it went.

Here's a quick guide how to start working with aeternum.

#### Select an image
Click on `Open Image` or drag and drop an image.

!!! failure
    Drag and dropping files currently doesn't work on Linux **with Wayland** yet.

![select image](select_image.webp)

#### Select a model
After selecting an image, you need to select a model to upscale with.
!!! info
    Aeternum comes with models by default.

![select model](select_model.png)

#### Change scale
Change the scale if needed.

![change scale](change_scale.png)

#### Change compression
Change the compression if needed.

![change compression](change_compression.png)

#### Change export folder
Change the export folder if needed.
!!! info
    if you don't specify one, it'll be put in the same directory where the image you imported is.

![change output](change_output.png)

#### Upscale
Press the `Upscale` button.

![upscale](upscale.webp)
