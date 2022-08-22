import fetch from "node-fetch"
import { Pool } from 'piqel'

(async() => {
    const r = await fetch(
        "https://registry.npmjs.org/-/v1/search?text=query"
    )
    const d = await r.json()
    const pool = Pool.new(JSON.stringify(d))
    const dataStr = pool.query(`
SELECT
  objects.package.name,
  objects.searchScore AS score 
ORDERED BY score
`)
    console.log(JSON.parse(dataStr))
})()