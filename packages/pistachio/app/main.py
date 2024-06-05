from fastapi import FastAPI, HTTPException
from pydantic import BaseModel
from typing import List

app = FastAPI()

fake_movie_dp = [
    {
        'name': 'Starwars',
        'plot': 'In a land not so far away.... lol'
    }
]

class Movie(BaseModel):
    name: str
    plot: str


@app.get('/', response_model=list[Movie])
async def index():
    return fake_movie_dp

@app.post('/', status_code = 201)
async def add_movie(payload: Movie):
    movie = payload.dict()
    fake_movie_dp.append(movie)
    return {'id': len(fake_movie_dp) - 1}

@app.delete('/{id}')
async def delete_movie(id: int):
    movie_length = len(fake_movie_dp)
    if 0 <= id <= movie_length:
        del fake_movie_dp[id]
        return None
    raise HTTPException(status_code=404, detail="Movie ID no exist")