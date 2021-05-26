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

const input = `
{
  "name": "partiql-pokemon",
  "version": "0.202105.0",
  "private": true,
  "license": "MIT",
  "array": [
    1,
    {
      "@material-ui/core": "^4.11.3",
      "@material-ui/icons": "^4.11.2"
    }
  ],
  "scripts": {
    "dev": "next",
    "build": "next build",
    "start": "next start",
    "prod": "next build && next start",
    "lint": "eslint . --fix -c .eslintrc.js --ext js,jsx,ts,tsx --ignore-pattern='!.*'",
    "type-check": "tsc"
  }
}
`
