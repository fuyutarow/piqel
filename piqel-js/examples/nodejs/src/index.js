const { Pool } = await
import ('piqel')

const data = [{
        name: "Rust",
        info: {
            since: 2010,
            extension: "rs",
        }
    },
    {
        name: "JavaScript",
        info: {
            since: 2000,
            extension: "js",
        }
    },
    {
        name: "Python",
        info: {
            since: 1995,
            extension: "py",
        }
    }
]

const pool = Pool.new(JSON.stringify(data))
const res = pool.query('SELECT name, info.extension AS ext')
console.log(res)