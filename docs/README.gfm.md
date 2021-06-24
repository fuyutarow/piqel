<div align="center">

<h1>
<code>partiql-rs</code>
</h1>

<strong>An implementation of PartiQL written in Rust</strong>

<h3>
<a href="https://partiql.vercel.app">Document</a>
</h3>

</div>

``` toml:tests-make/hello.toml
[tests.hello]
script = '''
cat<<EOS | pq -q "SELECT NAME, LOGNAME" -t json
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

## Familiy

| content                                                                     | lang              | package                                  |
|-----------------------------------------------------------------------------|-------------------|------------------------------------------|
| [pq](https://github.com/fuyutarow/partiql-rs/blob/alpha/src/bin/pq.rs)      | CLI (brew, scoop) |                                          |
| [partiql-rs](https://github.com/fuyutarow/partiql-rs)                       | Rust (cargo)      | https://crates.io/crates/partiql-rs      |
| [partiql-js](https://github.com/fuyutarow/partiql-rs/tree/alpha/partiql-js) | JavaScript (npm)  | https://www.npmjs.com/package/partiql-js |
| [partiql-py](https://github.com/fuyutarow/partiql-rs/tree/alpha/partiql-py) | Python (pip)      | https://pypi.org/project/partiql         |

## Table of Contants

-   [Features](#Features)
-   [Motivation](#Motivation)
-   [Usage](#Usage)
    -   [converting file format]()
-   [Installation](#Installation)
-   [Test](#Test)
-   [LICNECE](#LICENCE)

## Features

## Motivation

What’s [PartiQL](https://partiql.org/)?

## Usage

    pq samples/pokemon.json -q "$(cat<<EOF
    SELECT
      no AS id,
      name,
      weight/height/height AS bmi
    ORDER BY bmi DESC
    LIMIT 10
    EOF
    )" -t csv

    curl -s https://api.github.com/users/fuyutarow/repos | pq -q "$(cat<<EOS
    SELECT
      owner.login AS user,
      stargazers_count AS star,
      svn_url AS url,
    EOS
    )" -t yaml

## Installation

``` sh:$
brew install fuyutarow/tap/pq
```

``` sh:$
scoop install pq
```

### Convert JSON \<–> TOML \<–> YAML \<–> …

Support - \[x\] JSON - \[ \] JSON5 - \[x\] TOML - \[x\] YAML - \[x\] XML

    env | jo | pq
    env | jo | pq -t yaml
    env | jo | pq -t yaml | pq -t toml

sort keys of objects on output

    env | jo | pq -S ;:

jo[1]

### Convert data

    env | jo | pq "SELECT NAME AS name, USER AS user"

`ip` command is only available in Linux and WSL, not in Mac.

    ip -j -p | pq "$(cat<<EOS
    SELECT
      address,
      info.family AS inet,
      info.local
    FROM addr_info AS info
    WHERE inet LIKE 'inet%'
    EOS
    )"

-   [x] SELECT
-   [x] FROM
-   [x] LEFT JOIN
-   [x] WHERE
-   [x] LIKE
-   [x] ORDER BY
-   [x] LIMIT

[more
examples](https://github.com/fuyutarow/partiql-rs/tree/alpha/tests-make)

# Test

Use [tests-make](https://github.com/fuyutarow/tests-make) to test CLI
`pq`.

``` sh:$
brew install fuyutarow/tap/tests-make
tests-make tests-make/index.toml
makers test:pq
```

| content                                                                     | test                                                                        |
|-----------------------------------------------------------------------------|-----------------------------------------------------------------------------|
| [pq](https://github.com/fuyutarow/partiql-rs/blob/alpha/src/bin/pq.rs)      | [test](https://github.com/fuyutarow/partiql-rs/tree/alpha/tests-make)       |
| [partiql-rs](https://github.com/fuyutarow/partiql-rs)                       | [test](https://github.com/fuyutarow/partiql-rs/tree/alpha/tests)            |
| [partiql-js](https://github.com/fuyutarow/partiql-rs/tree/alpha/partiql-js) | [test](https://github.com/fuyutarow/partiql-rs/tree/alpha/partiql-js/tests) |
| [partiql-py](https://github.com/fuyutarow/partiql-rs/tree/alpha/partiql-py) | [test](https://github.com/fuyutarow/partiql-rs/tree/alpha/partiql-py/tests) |

## code coverage

``` sh:
cargo install cargo-kcov
cargo kcov
```

or

``` sh:$
makers cov
```

## For Development

Requirements - [cargo-make](https://github.com/sagiegurari/cargo-make)
for `makers`

### Preparation

    makers install-dev

### build

    makers build
    makers build:pq ;: for pq commnad

### test

    makers test:lib
    makers test:pq ;: for pq commnad

# LICENCE

[1] https://github.com/jpmens/jo
