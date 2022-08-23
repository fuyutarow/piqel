
// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'
import { Pool } from 'piqel'

export default async (req: NextApiRequest, res: NextApiResponse<unknown>) => {
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