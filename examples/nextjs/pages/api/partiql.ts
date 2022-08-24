
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
const cors = (clientUrl: string) => {
  const whitelists = [
    "http://localhost",
    'https://piqel.pages.dev',
  ]

  if (whitelists.some(pattern => clientUrl.startsWith(pattern))) {
    return initMiddleware(
      Cors({
        // Only allow requests with GET, POST and OPTIONS
        methods: ['GET'],
        origin: clientUrl,
      })
    )
  } else {
    return () => { }
  }
}


export default async (req: NextApiRequest, res: NextApiResponse<unknown>) => {
  await cors(req.headers.origin ?? "")(req, res)


  const { url, q: query } = req.query as {
    url: string
    q: string
  }
  const r = await fetch(url)
  const d = await r.json()
  const data = Pool.new(JSON.stringify(d)).query(query) ?? ""

  res.status(200).json(
    JSON.parse(data)
  )
}