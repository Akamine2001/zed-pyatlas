# zed-pyatlas

Zed extension for [pyatlas](https://github.com/Akamine2001/pyatlas), a Python auto-import LSP companion for basedpyright.

The extension registers `pyatlas` as a language server in Zed and downloads the matching prebuilt binary for your platform from the pyatlas GitHub Releases.

## Installation

### From the Zed extensions marketplace

Open the Zed command palette (`cmd+shift+p`), run `zed: extensions`, search for `pyatlas`, and click Install.

### As a dev extension (for local development)

1. Clone this repository
2. Open the Zed command palette (`cmd+shift+p`) and run `zed: install dev extension`
3. Select the cloned directory
4. Zed will build the extension for the `wasm32-wasip1` target and load it

## Configuration

Enable `pyatlas` alongside basedpyright in your `.zed/settings.json`:

```jsonc
{
  "languages": {
    "Python": {
      "language_servers": ["basedpyright", "pyatlas", "!pyright", "!pylsp", "ruff"],
      "show_completions_on_input": true
    }
  }
}
```

basedpyright handles type checking, hover, and go-to-definition, while pyatlas handles auto-import completion.

### Binary resolution

On first launch the extension resolves the `pyatlas` binary in the following order:

1. `lsp.pyatlas.binary.path` in settings, if set
2. `pyatlas` found on `$PATH`
3. Prebuilt binary for your platform, downloaded from the [pyatlas GitHub Releases](https://github.com/Akamine2001/pyatlas/releases) and cached under the extension's work directory

### Overriding the binary

To point the extension at a specific binary (for example a local release build):

```jsonc
{
  "lsp": {
    "pyatlas": {
      "binary": {
        "path": "/absolute/path/to/pyatlas",
        "arguments": ["lsp"]
      }
    }
  }
}
```

## Supported platforms

Binaries are published for:

- macOS (aarch64 / Apple Silicon)
- Linux (x86_64, aarch64)
- Windows (x86_64)

Intel Macs are not currently supported; set `lsp.pyatlas.binary.path` in your Zed settings to point at a locally built binary if you need to run on that platform.

## License

MIT License. See [LICENSE](./LICENSE) for details.
