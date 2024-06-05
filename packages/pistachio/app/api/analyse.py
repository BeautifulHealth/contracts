from fastapi import HTTPException, Header, APIRouter, File, UploadFile
from typing import List
import uuid

import os,sys
sys.path.insert(0,"..")
from glob import glob
import matplotlib.pyplot as plt
import numpy as np
import argparse
import skimage, skimage.io
import pprint
import torch
import torch.nn.functional as F
import torchvision, torchvision.transforms
import torchxrayvision as xrv
import json

from app.api.models import Analyse

IMAGE_DIR = "images/"

fake_analyse_db = [
]
# fake_analyse_db = [
#     {
#         'name': 'Starwars Episode 2',
#         'id': 1,
#         'analysis_hash': '0x001'
#     }
# ]

analyse = APIRouter()

@analyse.get('/analyse')
async def index():
    return fake_analyse_db

@analyse.post('/analyse', status_code = 201)
async def add_analysis_job(file: UploadFile = File(...)):
    # image = payload.dict()
    # fake_analyse_db.append(image)

    file.filename = f"{uuid.uuid4()}.jpg"
    contents = await file.read()
 
    #save the file
    with open(f"{IMAGE_DIR}{file.filename}", "wb") as f:
        f.write(contents)

    file_path = f"{IMAGE_DIR}{file.filename}"

    img = skimage.io.imread(file_path)
    img = xrv.datasets.normalize(img, 255) # convert 8-bit image to [-1024, 1024] range
    img = img.mean(2)[None, ...] # Make single color channel

    transform = torchvision.transforms.Compose([xrv.datasets.XRayCenterCrop(),xrv.datasets.XRayResizer(224)])

    img = transform(img)
    img = torch.from_numpy(img)

    # Load model and process image
    model = xrv.models.DenseNet(weights="densenet121-res224-all")
    outputs = model(img[None,...]) # or model.features(img[None,...]) 

    # # Print results
    outputs = dict(zip(model.pathologies,outputs[0].detach().numpy()))
    print(outputs)

    fake_analyse_db.append(outputs)

    return {"filename": file_path}
    # return dictionary

