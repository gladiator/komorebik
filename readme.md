# Komorebik

A configuration and keyboard shortcut handler for [komorebi](https://github.com/LGUG2Z/komorebi/).

## How to Use

A better solution will be provided at some point.  I've just been launching `komorebik` in the background using PowerShell.

```shell
Start-Process komorebik.exe -WindowStyle hidden
```

This assumes that `komorebik.exe` currently resides somewhere in `PATH`.

## Configuration

A sample configuration has been provided in `examples/`.  It is heavily based off of the `komorebi.generated.ahk` script.

This configuration executes as soon as *komorebik* has started, in order for changes to take effect you must restart the application.

### Keys

Currently, the modifier keys are `CONTROL+ALT`.  So far this has the least amount of conflicting usage with Windows and other applications but should be configurable in the future.

Keys are structured like komorebi's socket schema.  This will be explained in detail in the future, hopefully you can figure it out for now.

```toml
[keys]
left = { type = "MoveWindow", content = "Left" }
right = { type = "MoveWindow", content = "Right" }
up = { type = "MoveWindow", content = "Up" }
down = { type = "MoveWindow", content = "Down" }
```

Key names are determined by [keyboard.rs](src/keyboard.rs).  Unless explicitly renamed, use the enumerator's name in all lowercase.

### Windows

Window rules are specified within the configuration.

```toml
[[window]]
bordered = true     # identify-border-overflow-application
floating = true     # float-rule
layered = true      # identify-layered-application
managed = true      # manage-rule
name_change = true  # identify-object-name-change-application
tray = true         # identify-tray-application
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

This provides a very generic layout so almost any window can be configured.
