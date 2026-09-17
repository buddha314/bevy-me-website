# bevy-me-website

A minimal Bevy + Trunk web app scaffold for experimenting with multiple 3D
viewports blended with flat HTML content on the same page.

## What's included

- A Rust Bevy app wired to render into `#bevy-canvas`
- Two side-by-side 3D camera viewports in a single Bevy application
- A flat HTML/CSS overlay shell for page copy and viewport labels
- OpenSpec initialization for GitHub Copilot agent workflows
- Copilot setup steps that install OpenSpec and the Rust web build tooling

## Local development

Install the web target and Trunk once:

```bash
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
```

Run the web app:

```bash
trunk serve
```

Useful checks:

```bash
cargo check
cargo test
cargo check --target wasm32-unknown-unknown
```
