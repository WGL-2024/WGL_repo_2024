# wg_repo_2024

## Contributing
Please read [CONTRIBUTE.md](CONTRIBUTE.md)

## Usage
_Cargo.toml_
```toml
[dependencies]
wg_2024 = { git = "https://github.com/WGL-2024/WGL_repo_2024.git", features = ["serialize"] }
```
if you don't want serde remove the features attribute

Note that this repo is unstable and due to the volume of PR there will be a lot of breaking changes.  Thus it's important to update this dependency frequently. Cargo does not auto-update the dependencies
> Once a `git` dependency has been added, Cargo will lock that dependency to the latest commit at the time. New commits will not be pulled down automatically once the lock is in place. However, they can be pulled down manually with `cargo update`.

To get the latest commit in your projects make sure you run `cargo update`

If you want to use the test module add debug to features like this
_Cargo.toml_
```toml
[dependencies]
wg_2024 = { git = "https://github.com/WGL-2024/WGL_repo_2024.git", features = ["debug"] }
```