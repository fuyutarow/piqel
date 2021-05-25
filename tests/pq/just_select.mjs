#!/usr/bin/env zx
import { strict as assert } from 'assert'

const exptected = `[
  {
    "employeeName": "Bob Smith",
    "id": 3,
    "title": null
  },
  {
    "employeeName": "Susan Smith",
    "id": 4,
    "title": "Dev Mgr"
  },
  {
    "employeeName": "Jane Smith",
    "id": 6,
    "title": "Software Eng 2"
  }
]
`

// assert.equal((await $`
// cat samples/q1.json | ./target/debug/pq "$(cat<<EOS
// SELECT e.id,
//        e.name AS employeeName,
//        e.title AS title
// FROM hr.employees e
// EOS
// )" -t json | jq -S
// `).stdout, exptected)

// assert.equal((await $`
// cat samples/q1.json | ./target/debug/pq "$(cat<<EOS
// SELECT e.id,
//        e.name AS employeeName,
//        e.title AS title
// FROM hr.employees AS e
// EOS
// )" -t json | jq -S
// `).stdout, exptected)

// assert.equal((await $`
// cat samples/q1.json | ./target/debug/pq "$(cat<<EOS
// SELECT e.id,
//        e.name AS employeeName,
//        e.title AS title
// FROM hr.employees AS e
// EOS
// )" -t json | jq -S
// `).stdout, exptected)

// assert.equal((await $`
// cat samples/q1.json | ./target/debug/pq "$(cat<<EOS
// SELECT hr.employees.id AS id,
//        hr.employees.name AS employeeName,
//        hr.employees.title AS title
// FROM hr
// EOS
// )" -t json | jq -S
// `).stdout, exptected)

// assert.equal((await $`
// cat samples/q1.json | ./target/debug/pq "$(cat<<EOS
// SELECT hr.employees.id AS id,
//        hr.employees.name AS employeeName,
//        hr.employees.title AS title
// FROM hr
// EOS
// )" -t json | jq -S
// `).stdout, exptected)

// assert.equal((await $`
// cat samples/q1.json | ./target/debug/pq "$(cat<<EOS
// SELECT hr.employees.id AS k,
//        hr.employees.name AS employeeName,
//        hr.employees.title
// FROM hr
// EOS
// )" -t json | jq -S
// `).stdout, exptected)

$`
env | jo | ./target/debug/pq -t json
`
