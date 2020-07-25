# Data Utility Belt

CLI set of utilities to automate small common everyday tasks operated on strings, arrays and objects.

Orientated towards JavaScript and React development, and still on it's very early stages.

## Installation
`cargo install data-utility-belt`

## Strings

Nothing to see here yet.

## Array

Operations performed on arrays.

### Remove duplicates (default: true)

By default any array passed in will remove duplicate values.
```{bash}
$ data-utility-belt array "cli os os cli"

> cli, os
```


### props to array (React)

```{bash}
$ data-utility-belt array "
  goal={goal} strategy={strategy} idealinfluencer={idealinfluencer}
" props_to_array
  

> goal, idealinfluencer, strategy
```

### array to props (React) 

```{bash} 
data-utility-belt array "
  className,
  entityType,
  entityId,
  childType,
  childKey,
  chatContext,
  counts,
" array_to_props

> chatContext={chatContext} 
  childKey={childKey}
  childType'={childType'}
  counts={counts}
  entityId={entityId}
  entityType={entityType}
``` 

## Difference
```{bash}
cargo run -- object --difference  "$(cat examples/a.json )" "$(cat examples/b.json )"
```
