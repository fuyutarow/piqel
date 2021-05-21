# PartiQL-rs

WIP

```
$ cat 
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
$ cat samples/q1.env | cargo run --bin partiql-cli from --to json | jq
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
$ cat samples/q1.env | cargo run --bin partiql-cli from --to json | cargo run --bin partiql-cli from --to partiql
``` 