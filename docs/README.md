<div align="center">
  <div>
    <img src="https://raw.githubusercontent.com/fuyutarow/piqel/alpha/docs/static/img/label.png"></img>
  </div>
  <strong>An implementation of PartiQL written in Rust</strong>
  <h3>
    <a href="https://partiql.vercel.app">Document(WIP)</a>
  </h3>
</div>

```toml:tests-make/hello.toml
[tests.hello]
script = '''
cat<<EOS | pq -q "SELECT NAME, LOGNAME" -t json
{
  "SHELL": "/bin/bash",
  "NAME": "my machine name",
  "PWD": "/home/fuyutarow/piqel",
  "LOGNAME": "fuyutarow",
  "HOME": "/home/fuyutarow",
  "LANG": "C.UTF-8",
  "USER": "fuyutarow",
  "HOSTTYPE": "x86_64",
  "_": "/usr/bin/env"
}
EOS
'''
tobe = '''
[
  {
    "NAME": "my machine name",
    "LOGNAME": "fuyutarow"
  }
]
'''
```

## Family

| content | lang | package |
| --- | --- | --- |
| [pq](https://github.com/fuyutarow/piqel/blob/alpha/src/bin/pq.rs) | CLI (brew, scoop) | |
| [piqel](https://github.com/fuyutarow/piqel) | Rust (cargo) | https://crates.io/crates/piqel |
| [piqel-js](https://github.com/fuyutarow/piqel/tree/alpha/piqel-js) | JavaScript (npm) | https://www.npmjs.com/package/piqel |
| [piqel-py](https://github.com/fuyutarow/piqel/tree/alpha/piqel-py) | Python (pip) | https://pypi.org/project/piqel |



## Table of Contants
- [Features](#Features)
- [Motivation](#Motivation)
- [Usage](#Usage)
  - [pretty print](#pretty-print)
  - [convert file format](#convert-file-format)
  - [calculate BMI](#calculate-BMI)
- [Installation](#Installation)
- [Test](#Test)
- [LICNECE](#LICENCE)


## Features


## Motivation
What's [PartiQL](https://partiql.org/)?


## Usage

### pretty print


| option | description |
| --- | --- |
-c, --compact      | compact instead of pretty-printed output, only when outputting in JSON
-S, --sort-keys    | sort keys of objects on output. it on works when --to option is json, currently


```sh:$
curl -s "https://api.github.com/repos/fuyutarow/piqel/commits?per_page=1" | pq
```




### convert file format

| option | description |
| --- | --- |
-f, --from <from>      | target config file [possible values: csv, json, toml, yaml, xml]
-t, --to <to>          | target config file [possible values: csv, json, toml, yaml, xml]


use `-t` option c to convert Json, Yaml, Toml, and XML to each other.

```sh:$
cat pokemon.json | pq -t yaml
```
```sh:$
cat pokemon.json | pq -t yaml | pq -t toml
```


Comparison with existing command yj[^yj]

| format | pq | yj |
| --- | --- | --- |
| JSON | ✅ | ✅ |
| TOML | ✅ | ⚠️*1 |
| YAML | ✅ | ✅ |
| XML | ✅ | ✅ |
| CSV | ✅ | ❌ |

*1 TOML of the following format cannot be serialized with `yj`, but it can be serialized with `pq` by replacing the fields accordingly.

```json:pakcge.json
{
  "name": "partiql-pokemon",
  "dependencies": {
    "react": "^16.13.1",
    "react-dom": "^16.13.1"
  },
  "license": "MIT"
}
```

| option | description |
| ---- | ---- |
| `-q` | クエリ |

| query | description |
| --- | --- |
| `SELECT <field_path>` |
| `SELECT <field_path> AS <alias_path>` |



### Calculate BMI

1. Download the file and then calculate BMI in a local.
  ```sh:$
  curl -s https://raw.githubusercontent.com/fuyutarow/pokemon.json/master/en/pokemon.json | pq -q "SELECT name, weight/height/height AS bmi ORDER BY bmi DESC LIMIT 20"
  ```

2. In a terminal, send a query to the server to calculate BMI in a remote.
  ```sh:$
  curl https://partiql-pokemon.vercel.app/api/pokemon/ja -G --data-urlencode "q= SELECT name, weight/height/height AS bmi ORDER BY bmi DESC LIMIT 20"
  ```

3. In a web browser, send a query to the server to calculate BMI in a remote.

<a href="https://partiql-pokemon.vercel.app/api/pokemon/ja?q=%20SELECT%20name,%20weight/height/height%20AS%20%20bmi%20ORDER%20BY%20bmi%20DESC%20LIMIT%2020">
partiql-pokemon.vercel.app/api/pokemon/en?q= SELECT name, weight/height/height AS  bmi ORDER BY bmi DESC LIMIT 20
</a>

## Installation

```sh:$
brew install fuyutarow/tap/pq
pq -h
```
```sh:$
scoop install pq
pq -h
```


### Convert data
```
env | jo | pq "SELECT NAME AS name, USER AS user"
```

`ip` command is only available in Linux and WSL, not in Mac.
```
ip -j -p | pq "$(cat<<EOS
SELECT
  address,
  info.family AS inet,
  info.local
FROM addr_info AS info
WHERE inet LIKE 'inet%'
EOS
)"
```

- [x] SELECT
- [x] FROM
- [x] LEFT JOIN
- [x] WHERE
- [x] LIKE
- [x] ORDER BY
- [x] LIMIT

[more examples](https://github.com/fuyutarow/piqel/tree/alpha/tests-make)


## Test

Use [tests-make](https://github.com/fuyutarow/tests-make) to test CLI `pq`.

```sh:$
brew install fuyutarow/tap/tests-make
tests-make tests-make/index.toml
```
or
```sh:$
makers test:pq
```

| content | test | command |
| --- | --- | --- |
| [pq](https://github.com/fuyutarow/piqel/blob/alpha/src/bin/pq.rs) | [test](https://github.com/fuyutarow/piqel/tree/alpha/tests-make) | `makers test:pq` |
| [piqel](https://github.com/fuyutarow/piqel) | [test](https://github.com/fuyutarow/piqel/tree/alpha/tests) | `makers test:lib` |
| [piqel-js](https://github.com/fuyutarow/piqel/tree/alpha/piqel-js) | [test](https://github.com/fuyutarow/piqel/tree/alpha/piqel-js/tests) | `makers test:js` |
| [piqel-py](https://github.com/fuyutarow/piqel/tree/alpha/piqel-py) | [test](https://github.com/fuyutarow/piqel/tree/alpha/piqel-py/tests) | `makres test:py` |
| all | | `makers test` |


## code coverage
```sh:
cargo install cargo-kcov
cargo kcov
```
or
```sh:$
makers cov
```



### Preparation
```
makers install-dev
```

### build
```
makers build
makers build:pq ;: for pq commnad
```


## LICENCE



## Appendix

### Comparison of tools that can extract fields

jq[^jq] approach

```sh:$
curl -s "https://api.github.com/repos/fuyutarow/piqel/commits?per_page=1" | jq  ".[].commit.author"
```

gron[^gron] approach
```sh:$
curl -s "https://api.github.com/repos/fuyutarow/piqel/commits?per_page=1" | gron | grep "commit.author" | gron -u
```

nusehll[^nushell] approach
```sh:nu$
curl -s "https://api.github.com/repos/fuyutarow/piqel/commits?per_page=1" | from json | get commit.author | to json
```

pq[^pq] approach
```sh:$
curl -s "https://api.github.com/repos/fuyutarow/piqel/commits?per_page=1" | pq -q "SELECT commit.author"
```

### utils
- makers[^cargo-make]


[^cargo-make]: https://github.com/sagiegurari/cargo-make ... Run `cargo install cargo-make` to use `makers` commnad.
[^tests-make]: https://github.com/fuyutarow/tests-make ...
[^yj]: https://github.com/sclevine/yj
[^jq]: https://github.com/stedolan/jq
[^gron]: https://github.com/tomnomnom/gron
[^nushell]: https://github.com/nushell/nushell
[^tests-make]: https://github.com/fuyutarow/tests-make
[^pq]: https://github.com/fuyutarow/piqel
[^partiql-pokemon]: https://github.com/fuyutarow/partiql-pokemon
[^cargo-distribute]: https://github.com/fuyutarow/cargo-disritubte
[^pokemon.json]: https://github.com/fuyutarow/pokemon.json
[^structured-text-tools]: https://github.com/dbohdan/structured-text-tools
[^partiql.org]: https://partiql.org
[^partiql-spec]: https://github.com/partiql/partiql-spec https://partiql.org/assets/PartiQL-Specification.pdf
[^DynamoDB]: https://partiql.org/https://docs.aws.amazon.com/ja_jp/amazondynamodb/latest/developerguide/ql-reference.html
[^PartiQL-QLDB]: https://docs.aws.amazon.com/ja_jp/qldb/latest/developerguide/ql-reference.html
[^awesome-query-language]: https://github.com/fuyutarow/awesome-query-language