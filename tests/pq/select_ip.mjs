#!/usr/bin/env zx
import { strict as assert } from 'assert'


const expected = `[
  {
    "address": "00:00:00:00:00:00",
    "inet": "inet",
    "local": "127.0.0.1"
  },
  {
    "address": "00:00:00:00:00:00",
    "inet": "inet",
    "local": "::1"
  },
  {
    "address": "00:00:00:00:00:00",
    "inet": "inet6",
    "local": "127.0.0.1"
  },
  {
    "address": "00:00:00:00:00:00",
    "inet": "inet6",
    "local": "::1"
  },
  {
    "address": "00:16:4a:01:b1:cc",
    "inet": "inet",
    "local": "148.39.69.44"
  },
  {
    "address": "00:16:4a:01:b1:cc",
    "inet": "inet",
    "local": "de99::112:5dfd:de17:e1cf"
  },
  {
    "address": "00:16:4a:01:b1:cc",
    "inet": "inet6",
    "local": "148.39.69.44"
  },
  {
    "address": "00:16:4a:01:b1:cc",
    "inet": "inet6",
    "local": "de99::112:5dfd:de17:e1cf"
  }
]
`

assert.equal((await $`
cat samples/ip_addr.json | ./target/debug/pq -q "$(cat<<EOS
SELECT
  address,
  info.family AS inet,
  info.local
FROM addr_info AS info
WHERE info.family LIKE 'inet%'
EOS
)" -S
`).stdout, expected)

assert.equal((await $`
cat samples/ip_addr.json | ./target/debug/pq -q "$(cat<<EOS
SELECT
  address,
  addr_info.family AS inet,
  addr_info.local
WHERE addr_info.family LIKE 'inet%'
EOS
)" -S
`).stdout, expected)
