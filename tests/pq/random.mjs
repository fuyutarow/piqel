#!/usr/bin/env zx
import { strict as assert } from 'assert'

const expected = `[
  {
    "name": "Bob Smith"
  },
  {
    "name": "Susan Smith"
  },
  {
    "name": "Jane Smith"
  }
]
`

assert.equal((await $`
cat samples/q2.json | ./target/debug/pq -q "$(cat<<EOS
SELECT e.name AS name
FROM hr.employeesNest AS e
EOS
)" | jq -S
`).stdout, expected)

assert.equal((await $`
cat samples/q2.json | ./target/debug/pq -q "$(cat<<EOS
SELECT e.name
FROM hr.employeesNest AS e
EOS
)" | jq -S
`).stdout, expected)
