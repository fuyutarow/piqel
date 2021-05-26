#!/usr/bin/env zx
import { strict as assert } from 'assert'

const input = `
{
  "hr": {
    "employees": [
      {
        "id": 3,
        "name": "Bob Smith",
        "title": null
      },
      {
        "id": 4,
        "name": "Susan Smith",
        "title": "Dev Mgr"
      },
      {
        "id": 6,
        "name": "Jane Smith",
        "title": "Software Eng 2"
      }
    ]
  }
}
`

assert.equal((await $`
echo ${input} | ./target/debug/pq -t yaml
`).stdout, `---
hr:
  employees:
    - id: 3
      name: Bob Smith
      title: ~
    - id: 4
      name: Susan Smith
      title: Dev Mgr
    - id: 6
      name: Jane Smith
      title: Software Eng 2

`)

assert.equal((await $`
echo ${input} | ./target/debug/pq -t toml
`).stdout, `[[hr.employees]]
id = 3
name = 'Bob Smith'

[[hr.employees]]
id = 4
name = 'Susan Smith'
title = 'Dev Mgr'

[[hr.employees]]
id = 6
name = 'Jane Smith'
title = 'Software Eng 2'

`)

assert.equal((await $`
echo ${input} | ./target/debug/pq -t xml
`).stdout, `<hr><employees><id>3</id><name>Bob Smith</name><title></title><id>4</id><name>Susan Smith</name><title>Dev Mgr</title><id>6</id><name>Jane Smith</name><title>Software Eng 2</title></employees></hr>
`)


assert.equal((await $`
cat<<EOS | ./target/debug/pq -t toml
[
  {
    "id": 3,
    "name": "Bob Smith",
    "title": null
  },
  {
    "id": 4,
    "name": "Susan Smith",
    "title": "Dev Mgr"
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
id = 4
name = 'Susan Smith'
title = 'Dev Mgr'

[[]]
id = 6
name = 'Jane Smith'
title = 'Software Eng 2'

`)
