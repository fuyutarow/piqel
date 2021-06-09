import fs from 'fs';
import path from 'path'
import getConfig from 'next/config'
import type { NextApiRequest, NextApiResponse } from 'next'
const { serverRuntimeConfig } = getConfig()

import * as partiql from "partiql-js"

const pokemonJson = fs.readFileSync(
  path.join(serverRuntimeConfig.PROJECT_ROOT, 'samples/pokemon.json'),
  'utf8'
)

export default async (req: NextApiRequest, res: NextApiResponse<unknown>) => {
  const query = req.query.q ?? ""

  if (query) {
    try {
      let result = await partiql.evaluate(query as string, pokemonJson, "json", "json") ?? "";
      let data = JSON.parse(result);
      res.status(200).json(data);
    } catch (e) {
      res.status(400).json({ "message": e });
    }
  } else {
    res.status(400).json({ "message": "Require PartiQL for q query parameters" });
  }
}
