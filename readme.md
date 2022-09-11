# komorebik

A configuration and keyboard shortcut handler for [komorebi](https://github.com/LGUG2Z/komorebi/).

## How to Use

You can build from source using `cargo build --release`.

A better solution will be provided for starting komorebik in the background.  I've just been launching `komorebik.exe` using PowerShell.

```shell
Start-Process komorebik.exe -WindowStyle hidden
```

This assumes that komorebik currently resides somewhere in `PATH`.

## Configuration

komorebik's configuration is located at `~/.config/komorebik.toml`.

The configuration executes as soon as komorebik has started.  In order for changes to take effect you must restart the application.

A sample has been provided in `examples/`, it is heavily based off of the [komorebi.generated.ahk](https://github.com/LGUG2Z/komorebi/blob/master/komorebi.generated.ahk) script.

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

### Windows

```toml
[[window]]
bordered = true             # identify-border-overflow-application
floating = true             # float-rule
layered = true              # identify-layered-application
managed = true              # manage-rule
object_name_change = true   # identify-object-name-change-application
tray = true                 # identify-tray-application
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
