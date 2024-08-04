# rust-json-file-categorize

Tool for categorizing files that have annotations in json files

## Examples
Tree view:
```bash
.
├── Cargo.lock
├── Cargo.toml
├── config.json     # Config file
├── files
│   └── ...         # Files
├── labels
│   └── ...         # Json files
├── src
│   ├── config.rs
│   └── main.rs
```
Configuration file:
```json
{
    "file_dir": "./files",
    "label_dir": "./labels",
    "out_dir": "./out",
    "file_link_property": "filename",
    "categories": [
        {
            "name": "name",
            "filters": [
                {
                    "property": "key",
                    "values": [
                        "value1",
                        "value2"
                    ],
                },
            ]
        },
    ]
}
```
Run with:
```bash
cargo run config.json
```
