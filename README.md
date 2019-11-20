<img src="./screencap.gif" alt="Demo">

# Overview
tfex-rs is a simple \[t\]erminal \[f\]ile \[ex\]plorer written in Rust. It's not very useful in it's current state, and probably never will be. It was written for fun/practice rather than to be actually used.

# Controls
| Key | Command |
| --- | ------- |
| h | Move selection left |
| j | Move selection down |
| k | Move selection up |
| l | Move selection right |
| c | Copy file |
| x | Cut file |
| v | Paste file |
| : | Enter command mode |
| Esc | Exit command mode |
| Enter | Open folder or execute command |
| Backspace | Move up one directory |
| q | Quit |

# Working Commands
| Long | Short | Description |
|------|-------|-------------|
| :rename [new name]| :ren | Renames the selected file or directory |
| :delete | :del | Deletes the selected file or directory **[Dangerous - will delete all directory contents too. This is irreversible]**|
| :directory [name]| :dir | Creates a new directory |


# Installation
tfx-rs should definitely work on macOS. It'll *probably* work on Linux, and almost definitely won't work on Windows. 
* Install rustup (https://rustup.rs)
* Clone this repository (`git clone https://github.com/PorkSausages/tfex-rs.git`)
* Run `cargo install --path /path/to/cloned/repository/`
* Launch by running `tfex`