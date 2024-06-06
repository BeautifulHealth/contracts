from fastapi import FastAPI
from typing import List

from app.api.analyse import analyse

app = FastAPI()

app.include_router(analyse)