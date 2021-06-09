import type { NextApiRequest, NextApiResponse } from 'next'
import dynamic from 'next/dynamic'


import * as partiql from "partiql-js"


export default async (req: NextApiRequest, res: NextApiResponse<unknown>) => {
  const query = req.query.q;

  let n = await partiql.add(3, 3);
  // let n = 3;
  res.status(200).json({ query, n });

}
