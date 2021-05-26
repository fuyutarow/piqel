#!/usr/bin/env zx
import { strict as assert } from 'assert'

assert.equal((await $`
cat samples/pokemons.json | ./target/debug/pq -q "$(cat<<EOS
SELECT name, id
WHERE id='001'
EOS
)" -S
`).stdout, `[
  {
    "id": "001",
    "name": "Bulbasaur"
  }
]
`)
