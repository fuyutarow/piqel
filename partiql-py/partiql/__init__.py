from typing import Literal, Optional, Union, Any
import pandas as pd

from .partiql import __version__
from . import partiql


LangType = Literal["json", "yaml", "toml", "xml", "csv"]


def load(data):
    return DataLake(data)


def loads(s: str, from_type: LangType = "json"):
    data = partiql.loads(s, from_type)
    return DataLake(data)


class DataLake:
    data = None

    def __init__(self, data=None) -> None:
        self.data = data

    def __repr__(self) -> str:
        return repr(self.data)

    def query(self, q: str):
        self.data = partiql.query_evaluate(self.data, q)
        return self

    def to(self, to_type: LangType = "json") -> str:
        return partiql.dumps(self.data, to_type)

    def to_dict(self) -> dict:
        return self.data

    def from_df(self, df: pd.DataFrame):
        return loads(df.to_csv(), "csv")

    def to_df(self) -> pd.DataFrame:
        return pd.DataFrame(self.to_dict())
