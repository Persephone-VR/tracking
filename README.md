<p align="center" style="font-size: 2em"><em>persephone-vr / <strong>tracking</strong></em></p>

With Nix (Linux, macOS, or WSL) and direnv, just `direnv allow`

If you would like to switch versions, use `nix develop`

```
# Rust nightly (default):
nix develop

# Rust stable:
nix develop '.#stable'

# MSRV:
nix develop '.#msrv'
```

