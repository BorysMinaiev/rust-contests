# example-contests-workspace
How to use it:
- Switch default rust toolchain to nightly: 
    ```
    rustup default nightly
    ```
- Install rust-competitive-helper binary:
    ```
    cargo install --git https://github.com/rust-competitive-helper/rust-competitive-helper
    ```
- Fork this repository on github, clone it locally, open in CLion
- In CLion terminal run `rust-competitive-helper` from current directory

To use with [Competitive Companion](https://github.com/jmerle/competitive-companion):
- Add 4244 to custom ports in plugin
- Choose "Run listener" in `rust-competitive-helper`
- Click "Parse task" in plugin
- Project for this task will be created and opened in CLion.
- Testing should be done by running main.rs in corresponding crate
- Submit ./main/src/main.rs

# Other stuff

To make git not track changes in auto-generated main.rs file:
```
git update-index --assume-unchanged main/src/main.rs
```

If you want to use your version of rust-contest-helper, you can run it like this:
```
RUST_BACKTRACE=1 cargo run --manifest-path ../rust-competitive-helper/Cargo.toml 
```
