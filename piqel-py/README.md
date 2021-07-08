<div align="center">
  <div>
    <img src="https://raw.githubusercontent.com/fuyutarow/piqel/alpha/docs/static/img/label.png"></img>
  </div>
  <strong>An implementation of PartiQL written in Rust</strong>

  <h3>
    <a href="https://partiql.vercel.app">Document(WIP)</a>
  </h3>
</div>

## Installation
```sh:$
pip install partiql
```
```sh:$
poetry add partiql
```

## Usage
```py:test_partiql.py
import partiql as pq

data = {
    "SHELL": "/bin/bash",
    "NAME": "my machine name",
    "PWD": "/home/fuyutarow/piqel",
    "LOGNAME": "fuyutarow",
    "HOME": "/home/fuyutarow",
    "LANG": "C.UTF-8",
    "USER": "fuyutarow",
    "HOSTTYPE": "x86_64",
    "_": "/usr/bin/env",
}
dl = pq.DataLake(data)
dl = dl.query("SELECT NAME, LOGNAME")
output = dl.to("json")
expected = """[{"NAME":"my machine name","LOGNAME":"fuyutarow"}]"""
assert output == expected
```

## Test

```sh:$
makers test:py
```


## Family

| content | lang | package |
| --- | --- | --- |
| [pq](https://github.com/fuyutarow/piqel/blob/alpha/src/bin/pq.rs) | CLI (brew, scoop) | |
| [piqel](https://github.com/fuyutarow/piqel) | Rust (cargo) | https://crates.io/crates/piqel |
| [piqel-js](https://github.com/fuyutarow/piqel/tree/alpha/piqel-js) | JavaScript (npm) | https://www.npmjs.com/package/piqel |
| [piqel-py](https://github.com/fuyutarow/piqel/tree/alpha/piqel-py) | Python (pip) | https://pypi.org/project/piqel |

