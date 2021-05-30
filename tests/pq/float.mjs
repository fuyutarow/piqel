#!/usr/bin/env zx
import { strict as assert } from 'assert'

const  input = `{
  "name": "Mew",
  "id": 151,
  "fleeRate": 0.1
}
`

assert.equal((await $`
echo ${input} | ./target/debug/pq -t json
`).stdout, input)

assert.equal((await $`
echo ${input} | ./target/debug/pq -t toml
`).stdout, `name = 'Mew'
id = 151
fleeRate = 0.1

`)

assert.equal((await $`
echo ${input} | ./target/debug/pq -t yaml
`).stdout, `---
name: Mew
id: 151
fleeRate: 0.1

`)

assert.equal((await $`
echo ${input} | ./target/debug/pq -t xml
`).stdout, `<name>Mew</name><id>151</id><fleeRate>0.1</fleeRate>
`)
