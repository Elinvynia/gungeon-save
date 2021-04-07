[![ci-badge][]][ci]

# gungeon-save

A simple Enter the Gungeon save file decoder and encoder.

## Sample Usage
```bash
# If you are on Windows and want to use the default save path.
gungeon-save decode
# Otherwise, use
gungeon-save decode /path/to/save/file

# If you have a `SlotA.txt` in the current folder use
gungeon-save encode
# Otherwise, use
gungeon-save encode name-of-decoded-file

# Simple help message, explaining the same things as here.
gungeon-save help
```

[ci]: https://github.com/Elinvynia/gungeon-save/actions?query=workflow%3ARust
[ci-badge]: https://img.shields.io/github/workflow/status/Elinvynia/gungeon-save/Rust/master?style=flat-square
