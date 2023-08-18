# Differential Drive Robot based on BeagleBoneBlue

## Rust environment

* Install [rust cross](https://github.com/cross-rs/cross) for Cross-Compiling

## Build and Run

1. Build: `cross build --target arm-unknown-linux-gnueabihf`
2. Upload: `scp target/arm-unknown-linux-gnueabihf/debug/robot_diff_drive debian@192.168.7.2:bin/`
3. Run: `ssh debian@192.168.7.2 bin/robot_diff_drive`
