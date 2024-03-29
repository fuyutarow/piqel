# %%
from piqel import __version__
import piqel as pq
import pandas as pd


# %%
def test_version():
    assert __version__ == "0.202208.0"


# %%
def test_query_from_object():
    data = [
        {
            "name": "Python",
            "info": {
                "since": 1995,
                "extension": "py",
            },
        },
        {
            "name": "JavaScript",
            "info": {
                "since": 2000,
                "extension": "js",
            },
        },
        {
            "name": "Rust",
            "info": {
                "since": 2010,
                "extension": "rs",
            },
        },
    ]
    dl = pq.DataLake(data)
    dl = dl.query(
        """
SELECT
  name,
  info.since,
  info.extension AS ext
  ORDER BY since DESC
    """
    )
    assert dl.to_dict() == [
        {
            "name": "Rust",
            "since": 2010,
            "ext": 'rs'
        },
        {
            "name": "JavaScript",
            "since": 2000,
            "ext": 'js'
        },
        {
            "name": "Python",
            "since": 1995,
            "ext": 'py'
        }
    ]


def test_loads_json_and_query():
    pass
#     input = """[
#     {
#         "name": "Python",
#         "info": {
#             "since": 1995,
#             "extension": "py",
#         },
#     },
#     {
#         "name": "JavaScript",
#         "info": {
#             "since": 2000,
#             "extension": "js",
#         },
#     },
#     {
#         "name": "Rust",
#         "info": {
#             "since": 2010,
#             "extension": "rs",
#         },
#     },
# ]"""
#     pq.loads(input)
#     output = (
#         pq.loads(input)
#         .query(
#             """
# SELECT NAME, LOGNAME
# """
#         )
#         .to("json")
#     )
    # expected = """[{"NAME":"my machine name","LOGNAME":"fuyutarow"}]"""
    # assert output == expected


# %%
def test_csv():
    input = """Date,Open,High,Low,Close,Volume,Dividends,Stock Splits
2020-06-26,197.8146237204083,197.97309297836208,193.01114334930884,194.44723510742188,54675800,0.0,0
2020-06-29,193.90252285796703,196.62615112022135,191.6939121713373,196.53701782226562,26701600,0.0,0
2020-06-30,195.98239565459178,202.43985995118794,195.84373881662714,201.5583953857422,34310300,0.0,0
2020-07-01,201.19194661157007,204.37117030626138,199.83508934415127,202.7369842529297,32061200,0.0,0

"""
    output = (
        pq.loads(input, "csv")
        .query(
            """
SELECT Open
"""
        )
        .to("csv")
    )
    expected = """Open
197.8146237204083
193.90252285796703
195.98239565459176
201.19194661157007
"""
    assert output == expected


# %%
def test_dl2df2dl():
    with open("../samples/boston.csv") as f:
        csv_s = f.read()
    dl = pq.loads(csv_s, "csv")
    dl2 = pq.DataLake().from_df(dl.to_df())

    assert dl.to_dict() == dl2.to_dict()
