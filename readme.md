# komorebik

A configuration and keyboard shortcut handler for [komorebi](https://github.com/LGUG2Z/komorebi/).

---

## How to Use

You can build from source using `cargo build --release`.

A better solution will be provided for starting komorebik in the background.  I've just been launching `komorebik.exe` using PowerShell.

```shell
Start-Process komorebik.exe -WindowStyle hidden
```

This assumes that komorebik currently resides somewhere in `PATH`.

---

## Configuration

komorebik's configuration is located at `~/.config/komorebik.toml`.

The configuration executes as soon as komorebik has started.  In order for changes to take effect you must restart the application.

A sample has been provided in `examples/`, it is heavily based off of the [komorebi.generated.ahk](https://github.com/LGUG2Z/komorebi/blob/master/komorebi.generated.ahk) script.

---

### App-Specific Config Generation

komorebik has the ability to parse and generate a usable configuration from the [community's app-specific configuration](https://github.com/LGUG2Z/komorebi-application-specific-configuration).

There is an already generated example that may/may not be up-to-date within the `examples/` directory.

In order to generate a fresh one, pass an argument to `komorebik.exe` specifying a valid path to `applications.yaml`:

```shell
komorebik ./applications.yaml
# outputs: generated app-specific config at C:\Users\user\.config\komorebik.generated.toml!
```

This will create a file at `~/.config/komorebik.generated.toml`, the contents of which must be added to the bottom of your existing `komorebik.toml` configuration to take effect.

---

### Keys

Currently, the modifier keys are `CONTROL+ALT` as it has the least amount of conflicting usage but should be configurable in the future.

Keys are structured using komorebi's socket schema and should be explained more in the future.

```toml
[keys]
left = { type = "MoveWindow", content = "Left" }
right = { type = "MoveWindow", content = "Right" }
up = { type = "MoveWindow", content = "Up" }
down = { type = "MoveWindow", content = "Down" }
k = { type = "ResizeWindowAxis", content = ["Vertical", "Decrease"] }
i = { type = "ResizeWindowAxis", content = ["Vertical", "Increase"] }
j = { type = "ResizeWindowAxis", content = ["Horizontal", "Decrease"] }
l = { type = "ResizeWindowAxis", content = ["Horizontal", "Increase"] }
```

Key names are determined [here](src/keyboard.rs).  Unless explicitly renamed, write the name in "lower_snake_case".

---

### Window Configurations

Unique window configurations are supported.  These should be fairly self-explanatory if you've seen how `komorebic` handles certain configuration options.

```toml
[[window]]
# All of the following are optionally included, you may only need one option.
bordered = true      # identify-border-overflow-application
floating = true      # float-rule
layered = true       # identify-layered-application
managed = true       # manage-rule
name_change = true   # identify-object-name-change-application
tray = true          # identify-tray-application
[[window.rule]]
type = "class"
name = "SampleWindowClass"
[[window.rule]]
type = "exe"
name = "sample.exe"
[[window.rule]]
type = "title"
name = "Sample Title"
```
