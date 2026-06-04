# Advanced Configuration

!!! warning
    This section is for **advanced configuration**. You can configure Roseate with ease via the **[Settings Menu](../../misc/settings_menu.md)**. 

    Roseate is in alpha, expect sudden breaking changes in `config.toml`. 

The config file for the Roseate image viewer can be accessed and edited with the `--edit` or `-e` command:

=== "Linux"

    ```sh title="terminal"
    roseate --edit
    ```

=== "Windows"

    ```sh title="terminal"
    start roseate --edit
    ```

!!! note
    The section below is automatically generated from Roseate's toml config template which you can find here: **[https://github.com/cloudy-org/roseate/blob/main/app/assets/config.template.toml](https://github.com/cloudy-org/roseate/blob/main/app/assets/config.template.toml)**

    Hence to help edit this section you will need to edit the docstring in the toml template directly over at the Roseate repo.

!!! warning
    This section is in an experimental testing stage; it's very rough around the edges. Contributions to improve my code are welcome.

--8<-- "roseate-config-gen.md"