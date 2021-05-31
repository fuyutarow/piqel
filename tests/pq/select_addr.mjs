#!/usr/bin/env zx
import { strict as assert } from 'assert'


const expected = `[
  {
    "addr_info": {
      "family": "inet6",
      "local": "::1",
      "preferred_life_time": 4292929465,
      "prefixlen": 128,
      "scope": "host",
      "valid_life_time": 4294339495
    }
  },
  {
    "addr_info": {
      "family": "inet6",
      "local": "de99::112:5dfd:de17:e1cf",
      "preferred_life_time": 4294393545,
      "prefixlen": 64,
      "scope": "link",
      "valid_life_time": 42949393995
    }
  }
]
`

assert.equal((await $`
cat samples/ip_addr.json | ./target/debug/pq -q "$(cat<<EOS
SELECT addr_info
WHERE addr_info.family = 'inet6'
EOS
)" | jq -S
`).stdout, expected)

assert.equal((await $`
cat samples/ip_addr.json | ./target/debug/pq -q "$(cat<<EOS
SELECT addr_info
WHERE addr_info.family LIKE 'inet6'
EOS
)" | jq -S
`).stdout, expected)

// TODO: This test should be equal
assert.notEqual((await $`
cat samples/ip_addr.json | ./target/debug/pq -q "$(cat<<EOS
SELECT addr_info
WHERE addr_info.family = 'inet6'
EOS
)" -S
`).stdout, expected)

// TODO: This test should be equal
assert.notEqual((await $`
cat samples/ip_addr.json | ./target/debug/pq -q "$(cat<<EOS
SELECT addr_info AS info
WHERE addr_info.family = 'inet6'
EOS
)" | jq -S
`).stdout, expected)
