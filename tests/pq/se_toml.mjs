#!/usr/bin/env zx
import { strict as assert } from 'assert'

assert.equal((await $`
cat<<EOS | ./target/debug/pq -t toml
[
  {
    "id": 3,
    "name": "Bob Smith",
    "title": null
  },
  {
    "id": 6,
    "name": "Jane Smith",
    "title": "Software Eng 2"
  }
]
EOS
`).stdout, `[[]]
id = 3
name = 'Bob Smith'

[[]]
id = 6
name = 'Jane Smith'
title = 'Software Eng 2'

`)

assert.equal((await $`
cat<<EOS | ./target/debug/pq -t toml
{
  "name": "partiql-pokemon",
  "private": true,
  "scripts": {
    "dev": "next",
    "build": "next build",
    "start": "next start"
  },
  "license": "MIT"
}
EOS
`).stdout, `name = 'partiql-pokemon'
private = true
license = 'MIT'

[scripts]
dev = 'next'
build = 'next build'
start = 'next start'

`)
