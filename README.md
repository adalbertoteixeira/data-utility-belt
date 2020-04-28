# Data Utility Belt

## Array

### Remove duplicates (default: true)

```{bash}
$ data-utility-belt array "cli os os cli"

> cli, os
```

## Object


```{bash}
cargo run -- object --difference  "$(cat examples/a.json )" "$(cat examples/b.json )"
```
