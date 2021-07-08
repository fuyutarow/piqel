# piqel with Actix-web and DB

## Simply Run
```sh:$
cargo run
```

and accesss
<a href="http://localhost:8080/pokemon/en/partiql/SELECT%20name%20LIMIT%2010">
  http://localhost:8080/pokemon/en/partiql/SELECT name LIMIT 10
</a>


## Run with DB

### Requirements
- Docker

```sh:$
docker-compose up -d
```

or
```sh:$
makers up
```


```
makers db:create
makers db:migrate
```