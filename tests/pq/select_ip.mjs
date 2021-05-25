#!/usr/bin/env zx
import { strict as assert } from 'assert'

const input = `
[
  {
    "addr_info": [
      {
        "family": "inet",
        "label": "lo",
        "local": "127.0.0.1",
        "valid_life_time": 4294967295
      },
      {
        "family": "inet6",
        "local": "::1",
        "valid_life_time": 4294967295
      }
    ],
    "address": "00:00:00:00:00:00",
    "broadcast": "00:00:00:00:00:00",
    "flags": [
      "LOOPBACK",
      "UP",
      "LOWER_UP"
    ],
    "group": "default",
    "txqlen": 1000
  },
  {
    "addr_info": [],
    "address": "0.0.0.0",
    "broadcast": "0.0.0.0",
    "flags": [
      "NOARP"
    ],
    "group": "default",
    "txqlen": 1000
  },
  {
    "addr_info": [
      {
        "broadcast": "172.22.255.255",
        "family": "inet",
        "label": "eth0",
        "local": "172.22.247.125",
        "valid_life_time": 4294967295
      },
      {
        "family": "inet6",
        "local": "fe80::215:5dff:fed8:2bc4",
        "valid_life_time": 4294967295
      }
    ],
    "address": "00:15:5d:d8:2b:c4",
    "broadcast": "ff:ff:ff:ff:ff:ff",
    "flags": [
      "BROADCAST",
      "MULTICAST",
      "UP",
      "LOWER_UP"
    ],
    "group": "default",
    "txqlen": 1000
  }
]
`

const expected = `[
  {
    "address": "00:00:00:00:00:00",
    "inet": "inet",
    "local": "127.0.0.1"
  },
  {
    "address": "00:00:00:00:00:00",
    "inet": "inet6",
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
    "local": "::1"
  },
  {
    "address": "00:15:5d:d8:2b:c4",
    "inet": "inet",
    "local": "172.22.247.125"
  },
  {
    "address": "00:15:5d:d8:2b:c4",
    "inet": "inet6",
    "local": "172.22.247.125"
  },
  {
    "address": "00:15:5d:d8:2b:c4",
    "inet": "inet",
    "local": "fe80::215:5dff:fed8:2bc4"
  },
  {
    "address": "00:15:5d:d8:2b:c4",
    "inet": "inet6",
    "local": "fe80::215:5dff:fed8:2bc4"
  }
]
`

assert.equal((await $`
INPUT=$(cat<<EOS
${input}
EOS
)
echo $INPUT | ./target/debug/pq -q "$(cat<<EOS
SELECT
  address,
  info.family AS inet,
  info.local
FROM addr_info AS info
WHERE inet LIKE 'inet%'
EOS
)" | jq -S
`).stdout, expected)

assert.equal((await $`
INPUT=$(cat<<EOS
${input}
EOS
)
echo $INPUT | ./target/debug/pq -q "$(cat<<EOS
SELECT
  address,
  addr_info.family AS inet,
  addr_info.local
WHERE inet LIKE 'inet%'
EOS
)" | jq -S
`).stdout, expected)
