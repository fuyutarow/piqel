#!/usr/bin/env zx
import { strict as assert } from 'assert'

assert.equal((await $`
cat<<EOS | ./target/debug/pq -q "SELECT NAME, LOGNAME" -t json
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
`).stdout,
`[
  {
    "NAME": "my machine name",
    "LOGNAME": "fuyutarow"
  }
]
`)

assert.equal((await $`
cat<<EOS | ./target/debug/pq -q "SELECT NAME, LOGNAME" -t json -S
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
`).stdout,
`[
  {
    "LOGNAME": "fuyutarow",
    "NAME": "my machine name"
  }
]
`)
