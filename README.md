# Starlark / Bazel support for Zed

* [Starpls](https://github.com/withered-magic/starpls) LSP for starlark, bazel, and buck files.
* [Buck2](https://buck2.build/) LSP for buck files.
* Tree sitter syntax for [starlark](https://github.com/tree-sitter-grammars/tree-sitter-starlark) _and_ [bazelrc](https://github.com/zaucy/tree-sitter-bazelrc) files
* [Tilt](https://tilt.dev/) LSP for Tiltfiles.

## Configure the LSP

By default, the extension only loads the Starpls LSP. If your project uses Buck2 or Tilt, you must manually enable the corresponding LSP in your zed settings

To use buck2:

```json
{
  "languages": {
    "Starlark": {
      "language_servers": ["buck2-lsp", "!starpls", "!tilt"]
    }
  }
}
```

To use tilt:

```json
{
  "languages": {
    "Starlark": {
      "language_servers": ["tilt", "!starpls", "!buck2-lsp"]
    }
  }
}
```
