#!/usr/bin/env zx
import { strict as assert } from 'assert'

assert.equal((await $`
cat samples/ip_addr.json | ./target/debug/pq -q "SELECT address, ifname, addr_info.family" -t csv
`).stdout, `address,ifname,family
00:00:00:00:00:00,lo,inet
00:00:00:00:00:00,lo,inet6
25:86:1b:a7:46:a0,bond0,
0b:a7:8f:1c:5d:fa,dummy0,
0.0.0.0,sit0,
00:16:4a:01:b1:cc,eth0,inet
00:16:4a:01:b1:cc,eth0,inet6

`)
