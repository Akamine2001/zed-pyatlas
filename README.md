# pyatlas Zed extension

Zed 向けに `pyatlas` を language server として登録する最小拡張。

## ローカル dev extension として読み込む

1. Zed で `cmd+shift+P` → `zed: install dev extension`
2. このディレクトリ（`.../pyatlas/zed-extension/`）を選択
3. Zed が `wasm32-wasip1` ターゲットでビルドして読み込む

読み込み後、`.zed/settings.json` で `"pyatlas"` が language_server として有効になる。バイナリパスは `lsp.pyatlas.binary.path` で指定する（未指定時は `$PATH` の `pyatlas` を探す）。

## 設定例

```jsonc
{
  "languages": {
    "Python": {
      "language_servers": ["basedpyright", "pyatlas", "!pyright", "!pylsp", "ruff"]
    }
  },
  "lsp": {
    "pyatlas": {
      "binary": {
        "path": "/absolute/path/to/target/release/pyatlas"
      }
    }
  }
}
```
