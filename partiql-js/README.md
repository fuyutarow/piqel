<div align="center">
  <h1><code>partiql-js</code></h1>
  <strong>An implementation of PartiQL written in Rust</strong>

  <h3>
    <a href="https://partiql.vercel.app">Document</a>
  </h3>
</div>



## Installation
```sh:$
npm add partiql
```
```sh:$
yarn add partiql
```

## Usage
```js:test_partiql.py
import partiql

def test_evaluate():
    sql = "SELECT NAME, LOGNAME"
    input = """
{
  "SHELL": "/bin/bash",
  "NAME": "my machine name",
  "PWD": "/home/fuyutarow/partiql-rs",
  "LOGNAME": "fuyutarow",
  "HOME": "/home/fuyutarow",
  "LANG": "C.UTF-8",
  "USER": "fuyutarow",
  "HOSTTYPE": "x86_64",
  "_": "/usr/bin/env"
}
"""
    expected = """[{"NAME":"my machine name","LOGNAME":"fuyutarow"}]"""
    assert (
        partiql.evaluate(
            sql,
            input,
            "json",
            "json",
        )
        == expected
    )
```


# Familiy

| content | lang | package |
| --- | --- | --- |
| [pq](https://github.com/fuyutarow/partiql-rs/blob/alpha/src/bin/pq.rs) | CLI (brew, scoop) | |
| [partiql-rs](https://github.com/fuyutarow/partiql-rs) | Rust (cargo) | https://crates.io/crates/partiql-rs |
| [partiql-js](https://github.com/fuyutarow/partiql-rs/tree/alpha/partiql-js) | JavaScript (npm) | https://www.npmjs.com/package/partiql-js |
| [partiql-py](https://github.com/fuyutarow/partiql-rs/tree/alpha/partiql-py) | Python (pip) | https://pypi.org/project/partiql |
