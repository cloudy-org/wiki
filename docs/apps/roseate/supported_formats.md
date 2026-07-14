# Supported Image Formats
As of **`0.1.0-alpha.24`**, Roseate supports the following formats:

| Image Format | Support | Available in [Roseate Slim](./advanced_installs.md/#roseate-slim-aka-basic) | Additional Details |
| ----- | ----- | ----- | ----- |
| `PNG` | ✅ | ✅ | None |
| `JPEG` | ✅ | ✅ | None |
| `GIF` | ✅ | ✅ | None |
| `WEBP` | ✅ | ✅ | None |
| `QOI` | ✅ | ✅ | None |
| `AVIF` | 🟡 | 🟡 | Higher bit depth images can display incorrectly or washed out and the avif format is currently only available for Linux. |
| `BMP` | ✅ | ❌ | Not common enough to be included in the slim installation. |
| `ICO` | ✅ | ❌ | Not common enough to be included in the slim installation. |
| `TIFF` | 🟡 | ❌ | The [Image RS][image_rs_repo] decoder isn't 100% there with the tiff format, some images may fail. |
| `JPEG-XL` | ❌ | ❌ | A [Zune Image][zune_image_repo] backend is [planned](https://github.com/cloudy-org/roseate/issues/102) and should hopefully bring support for jpeg xl. |

> Icon Definitions:
> 
> | Columns | ✅ | 🟡 | ❌ |
> | ----- | ----- | ----- | ----- |
> | Support | Well supported format | Format partially support | Format not supported yet |
> | Roseate Slim | Available in slim install | Only available on some platforms | Not available in slim |

Those are the supported and planned to support formats as of version [**`0.1.0-alpha.24`**](https://github.com/cloudy-org/roseate/releases). Many more formats will be supported in future updates, code contributions are welcome in that area.