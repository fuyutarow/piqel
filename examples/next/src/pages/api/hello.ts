import type { NextApiRequest, NextApiResponse } from 'next'


export default (req: NextApiRequest, res: NextApiResponse<unknown>) => {
  const query = req.query.q;
  res.status(200).json({ query });

}
