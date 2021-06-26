# %%
from partiql import __version__
import partiql


# %%
def test_version():
    assert __version__ == "0.202106.7"


def test_json():
    sql = "SELECT NAME, LOGNAME"
    input = """
{
  "SHELL": "/bin/bash",
  "NAME": "my machine name",
  "PWD": "/home/fuyutarow/partiql-rs",
  "LOGNAME": "fuyutarow",
  "HOME": "/home/fuyutarow",
  "LANG": "C.UTF-8",
  "USER": "fuyutarow",
  "HOSTTYPE": "x86_64",
  "_": "/usr/bin/env"
}
"""
    expected = """[{"NAME":"my machine name","LOGNAME":"fuyutarow"}]"""
    assert (
        partiql.evaluate(
            input,
            sql,
            "json",
            "json",
        )
        == expected
    )


def test_csv():
    sql = "SELECT Open"
    input = """Date,Open,High,Low,Close,Volume,Dividends,Stock Splits
2020-06-26,197.8146237204083,197.97309297836208,193.01114334930884,194.44723510742188,54675800,0.0,0
2020-06-29,193.90252285796703,196.62615112022135,191.6939121713373,196.53701782226562,26701600,0.0,0
2020-06-30,195.98239565459178,202.43985995118794,195.84373881662714,201.5583953857422,34310300,0.0,0
2020-07-01,201.19194661157007,204.37117030626138,199.83508934415127,202.7369842529297,32061200,0.0,0

"""
    expected = """Open
197.8146237204083
193.90252285796703
195.98239565459176
201.19194661157007
"""
    assert (
        partiql.evaluate(
            input,
            sql,
            "csv",
            "csv",
        )
        == expected
    )
