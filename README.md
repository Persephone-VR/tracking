# _persephone-vr / tracking_

With Nix (Linux, macOS, or WSL) and [direnv](https://direnv.net/), just `direnv allow`

If you would like to switch versions, use `nix develop`

```zsh
# Rust nightly (default used by direnv):
nix develop

# Rust stable:
nix develop '.#stable'

# MSRV:
nix develop '.#msrv'
```
