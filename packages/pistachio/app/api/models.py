from typing import List
from pydantic import BaseModel

class Analyse(BaseModel):
    id: int
    name: str
    analysis_hash: str
