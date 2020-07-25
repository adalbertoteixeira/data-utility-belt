# Data Utility Belt

CLI set of utilities to automate small common everyday tasks operated on strings, arrays and objects.

Orientated towards JavaScript and React development, and still on it's very early stages-

## Installation
`cargo install data-utility-belt`

## Strings

Nothing to see here yet.

## Array

Operations performed on arrays.

### Remove duplicates (default: true)

By default any array passed in will remove duplicate values.
```{bash}
$ data_utility_belt array "cli os os cli"

> cli, os
```


### props to array (react)

```{bash}
$ data_utility_belt array "goal={goal} strategy={strategy} idealinfluencer={idealinfluencer}" props_to_array`

> goal, idealinfluencer, strategy
```

## Object

```{bash} 
data-utility-belt array "
  className: `${cbn}__actions__chat`,
  entityType: 'deliverable',
  entityId: id,
  childType: 'campaign',
  childKey: campaignId,
  chatContext: 'deliverable',
  counts: messagesCount,
" object_to_props
``` 


## Difference
```{bash}
cargo run -- object --difference  "$(cat examples/a.json )" "$(cat examples/b.json )"
```
