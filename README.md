# PartiQL-rs

WIP

What's [PartiQL](https://partiql.org/)?


# `pq` command
```
pq samples/pokemon.json -q "$(cat<<EOF
SELECT
  no AS id,
  name,
  weight/height/height AS bmi
ORDER BY bmi DESC
LIMIT 10
EOF
)" -t csv
```

```
curl -s https://api.github.com/users/fuyutarow/repos | pq -q "$(cat<<EOS
SELECT
  owner.login AS user,
  stargazers_count AS star,
  svn_url AS url,
EOS
)" -t yaml
```

## Installation
```
brew install fuyutarow/tap/pq
```

## Sample Usage

### Convert JSON <--> TOML <--> YAML <--> ...
Support
- [x] JSON
- [ ] JSON5
- [x] TOML
- [x] YAML
- [x] XML

```
env | jo | pq
env | jo | pq -t yaml
env | jo | pq -t yaml | pq -t toml
```

sort keys of objects on output
```
env | jo | pq -S ;:
```

##### FYI
- [jo](https://github.com/jpmens/jo) is a useful tool for creating json objects.
  ```
  brew install jo
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

[more examples](https://github.com/fuyutarow/partiql-rs/tree/alpha/tests-make)


## For Development
Requirements
- [cargo-make](https://github.com/sagiegurari/cargo-make) for `makers`

### Preparation
```
makers install-dev
```

### build
```
makers build
makers build:pq ;: for pq commnad
```

### test
```
makers test:lib
makers test:pq ;: for pq commnad
```