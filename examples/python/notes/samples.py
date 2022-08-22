# %%
import piqel as pq
import requests

# %%
r = requests.get("https://registry.npmjs.org/-/v1/search?text=query")
dl = pq.DataLake(r.json())
query = """
SELECT
  objects.package.name, 
  objects.searchScore AS score 
ORDERED BY score
"""
data = dl.query(query)
data


# %%
