#!/usr/bin/env zx
import { strict as assert } from 'assert'

assert.equal((await $`
cat<<EOS | ./target/debug/pq -q "address" | jq -S
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
EOS
`).stdout,
`[
  {
    "family": "inet",
    "label": "lo",
    "local": "127.0.0.1",
    "preferred_life_time": 4294967295,
    "prefixlen": 8,
    "scope": "host",
    "valid_life_time": 4294967295
  },
  {
    "family": "inet6",
    "local": "::1",
    "preferred_life_time": 4294967295,
    "prefixlen": 128,
    "scope": "host",
    "valid_life_time": 4294967295
  }
],
[
  {
    "broadcast": "172.22.255.255",
    "family": "inet",
    "label": "eth0",
    "local": "172.22.247.125",
    "preferred_life_time": 4294967295,
    "prefixlen": 20,
    "scope": "global",
    "valid_life_time": 4294967295
  },
  {
    "family": "inet6",
    "local": "fe80::215:5dff:fed8:2bc4",
    "preferred_life_time": 4294967295,
    "prefixlen": 64,
    "scope": "link",
    "valid_life_time": 4294967295
  }
]
`)
