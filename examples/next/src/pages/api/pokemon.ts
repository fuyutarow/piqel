import type { NextApiRequest, NextApiResponse } from 'next'

import * as partiql from "partiql-js"



const pokemon = `
[{"name":"Bulbasaur"},{"name":"Ivysaur"},{"name":"Venusaur"},{"name":"Charmander"},{"name":"Charmeleon"},{"name":"Charizard"},{"name":"Squirtle"},{"name":"Wartortle"},{"name":"Blastoise"},{"name":"Caterpie"},{"name":"Metapod"},{"name":"Butterfree"},{"name":"Weedle"},{"name":"Kakuna"},{"name":"Beedrill"},{"name":"Pidgey"},{"name":"Pidgeotto"},{"name":"Pidgeot"},{"name":"Rattata"},{"name":"Raticate"}]
`

export default async (req: NextApiRequest, res: NextApiResponse<unknown>) => {
  const query = req.query.q as string;

  let s = await partiql.evaluate(query, pokemon, "json", "json") ?? "";
  let data = JSON.parse(s);
  res.status(200).json({ data });

}
