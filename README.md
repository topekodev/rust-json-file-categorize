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
    "file_out_dir": "./files-out",
    "label_out_dir": "./labels-out",
    "file_link_property": "filename",
    "file_link_property_get_filename": false,
    "method": "symlink",
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
