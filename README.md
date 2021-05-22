# PartiQL-rs

WIP

What's [PartiQL](https://partiql.org/)?


## `partiql-cli`

### Installation
```
brew install fuyutarow/tap/partiql-cli
```

### Usage

#### `partiql-cli sql`
Using SQL to select data from JSON.
```
sql=$(cat << EOS
SELECT e.id,
       e.name AS employeeName,
       e.title AS title
FROM hr.employees e
WHERE e.title = 'Dev Mgr'
EOS
)
partiql-cli sql -q "$sql" -f samples/q1.json -t json | jq
```
```
[
  {
    "id": 4,
    "employeeName": "Susan Smith",
    "title": "Dev Mgr"
  }
]
```

#### `partiql-cli from`
Convert PartiQL-IR <--> JSON.

This is a PartiQL-IR.
```
$ cat samples/q1.env
{ 
    'hr': { 
        'employees': <<
            -- a tuple is denoted by { ... } in the PartiQL data model
            { 'id': 3, 'name': 'Bob Smith',   'title': null }, 
            { 'id': 4, 'name': 'Susan Smith', 'title': 'Dev Mgr' },
            { 'id': 6, 'name': 'Jane Smith',  'title': 'Software Eng 2'}
        >>
    }
} 
```


PartiQL-IR --> JSON
```sh
cat samples/q1.env | partiql-cli from --to json | jq
```
```json
{
  "hr": {
    "employees": [
      {
        "name": "Bob Smith",
        "title": null,
        "id": 3
      },
      {
        "name": "Susan Smith",
        "title": "Dev Mgr",
        "id": 4
      },
      {
        "name": "Jane Smith",
        "id": 6,
        "title": "Software Eng 2"
      }
    ]
  }
}
```

PartiQL-IR --> JSON --> PartiQL-IR
```sh
cat samples/q1.env | partiql-cli from --to json | partiql-cli from --to partiql
```
