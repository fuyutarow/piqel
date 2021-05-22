# PartiQL-rs

WIP


## `partiql-cli`

### Installation
```
brew install fuyutarow/tap/partiql-cli
```

### Usage

#### `partiql-cli sql`
```
$ cat << EOF | partiql-cli sql -q "$(cat)" -f samples/q1.json -t json
SELECT e.id,
       e.name AS employeeName,
       e.title AS title
FROM hr.employees e
WHERE e.title = 'Dev Mgr'
EOF
[{"employeeName":"Susan Smith","title":"Dev Mgr","id":4.0}]
```

#### `partiql-cli from`
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
$ cat samples/q1.env | partiql-cli from --to json | jq
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
$ cat samples/q1.env | partiql-cli from --to json | partiql-cli from --to partiql
```
