
// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import { Pool } from 'piqel'
import Cors from 'cors'

// Helper method to wait for a middleware to execute before continuing
// And to throw an error when an error happens in a middleware
// https://github.com/vercel/next.js/blob/canary/examples/api-routes-cors/lib/init-middleware.js
function initMiddleware(middleware: any) {
  return (req: NextApiRequest, res: NextApiResponse) =>
    new Promise((resolve, reject) => {
      middleware(req, res, (result: any) => {
        if (result instanceof Error) {
          return reject(result)
        }
        return resolve(result)
      })
    })
}

// Initialize the cors middleware
// https://github.com/vercel/next.js/blob/canary/examples/api-routes-cors/pages/api/cors.js
const cors = initMiddleware(
  // You can read more about the available options here: https://github.com/expressjs/cors#configuration-options
  Cors({
    // Only allow requests with GET, POST and OPTIONS
    methods: ['GET'],
    origin: 'https://piqel.pages.dev',
  })
)


export default async (req: NextApiRequest, res: NextApiResponse<unknown>) => {
  await cors(req, res)

  // @ts-ignore
  const params = new URLSearchParams(req.query);

  const url = params.get('url') ?? ""
  const query = params.get('q')
  const r = await fetch(url)
  const d = await r.json()
  const data = Pool.new(JSON.stringify(d)).query(query as string) ?? ""

  res.status(200).json(
    JSON.parse(data)
  )
}