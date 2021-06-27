from typing import Literal, Optional, Union

from .partiql import __version__
from . import partiql


LangType = Literal["json", "yaml", "toml", "xml", "csv"]


class DataLake:
    data = None

    def __init__(self) -> None:
        pass

    def __repr__(self) -> str:
        return repr(self.data)

    def load(self, data):
        self.data = data
        return self

    def loads(self, s: str, from_type: LangType = "json"):
        self.data = partiql.loads(s, from_type)
        return self

    def dumps(self, to_type: LangType = "json") -> str:
        return partiql.dumps(self.data, to_type)

    def query(self, q: str):
        self.data = partiql.query_evaluate(self.data, q)
        return self
