# Data Utility Belt

## Array

### Remove duplicates (default: true)

```{bash}
$ data_utility_belt array "cli os os cli"

> cli, os
```

## Object


```{bash}
cargo run -- object --difference  "$(cat examples/a.json )" "$(cat examples/b.json )"
```
