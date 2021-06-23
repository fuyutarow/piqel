from partiql import __version__
import partiql


def test_version():
    assert __version__ == "0.1.0"


def test_evaluate():
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
            sql,
            input,
            "json",
            "json",
        )
        == expected
    )
