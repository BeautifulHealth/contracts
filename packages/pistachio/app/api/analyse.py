from fastapi import HTTPException, Header, APIRouter, File, UploadFile
from typing import List
import uuid
import os
import sys

sys.path.insert(0, "..")
from glob import glob
import matplotlib.pyplot as plt
import numpy as np
import argparse
import skimage.io
import pprint
import torch
import torch.nn.functional as F
import torchvision.transforms
import torchxrayvision as xrv
import json

from app.api.models import Analyse

IMAGE_DIR = "images/"

# Fake database to store analysis results
fake_analyse_db = [
    {
        'name': 'Test-Patient-000001',
        'id': 1,
        'analysis_hash': '0x0000000001'
    }
]

# Router for the analysis endpoints
analyse = APIRouter()

@analyse.get('/analyse')
async def index():
    """
    Retrieve the list of all analysis jobs in the fake database.
    Returns:
        List of analysis jobs.
    """
    return fake_analyse_db

@analyse.post('/analyse', status_code=201)
async def add_analysis_job(file: UploadFile = File(...)):
    """
    Add a new analysis job by uploading an image file.
    The image is processed and analyzed using a pre-trained model.
    Args:
        file (UploadFile): The image file to be uploaded and analyzed.
    Returns:
        dict: A dictionary containing the filename of the saved image.
    """
    # Generate a unique filename for the uploaded image
    file.filename = f"{uuid.uuid4()}.jpg"
    contents = await file.read()

    # Save the uploaded file to the IMAGE_DIR directory
    with open(f"{IMAGE_DIR}{file.filename}", "wb") as f:
        f.write(contents)

    file_path = f"{IMAGE_DIR}{file.filename}"

    # Read and process the image
    img = skimage.io.imread(file_path)
    img = xrv.datasets.normalize(img, 255)  # Convert 8-bit image to [-1024, 1024] range
    img = img.mean(2)[None, ...]  # Make single color channel

    transform = torchvision.transforms.Compose([
        xrv.datasets.XRayCenterCrop(),
        xrv.datasets.XRayResizer(224)
    ])

    img = transform(img)
    img = torch.from_numpy(img)

    # Load pre-trained DenseNet model and process the image
    model = xrv.models.DenseNet(weights="densenet121-res224-all")
    outputs = model(img[None, ...])  # or model.features(img[None, ...])

    # Extract and format the model outputs
    outputs = dict(zip(model.pathologies, outputs[0].detach().numpy()))
    print(outputs)

    # Append the analysis results to the fake database
    fake_analyse_db.append(outputs)

    # Return the filename of the saved image
    return {"filename": file_path}
