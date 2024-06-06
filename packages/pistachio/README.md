## Chest X-ray Analysis Application (Pistachio)

This application provides an API for analyzing chest X-ray images using a pre-trained DenseNet model. It allows users to upload X-ray images, process them, and retrieve analysis results. The application uses FastAPI for building the API and various Python libraries for image processing and machine learning.

### Features
1. Upload X-ray Images: Upload chest X-ray images for analysis.
2. Image Processing: Normalize and preprocess images for analysis.
3. Deep Learning Model: Use a pre-trained DenseNet model to analyze the images.
4. Retrieve Analysis Results: Get the analysis results including various pathologies detected in the X-ray.

### Installation

#### Prerequisites
1. Python 3.7 or higher
2. Pip (Python package installer)


#### Install Dependencies
Create a virtual environment and activate it:

```
python -m venv venv
source venv/bin/activate  # On Windows, use `venv\Scripts\activate`

```

Install the required packages:
```
pip install -r requirements.txt
```

#### Run the API
To start the FastAPI server, run:
```
uvicorn app.main:app --reload
```

The API will be available at http://127.0.0.1:8000.
The APIdocs will be at http://127.0.0.1:8000/docs.

#### API Endpoints
Get All Analyses
1. Endpoint: /analyse
2. Method: GET
3. Description: Retrieve the list of all analysis jobs in the fake database.
4. Response: JSON array of analysis jobs.

```
{
    'name': 'Test-Patient-000001',
    'id': 1,
    'analysis_hash': '0x0000000001'
}
```


#### Add New Analysis Job
1. Endpoint: /analyse
2. Method: POST
3. Description: Add a new analysis job by uploading an X-ray image.
4. Request: Multipart form-data containing the image file.
5. Response: JSON object containing the filename of the saved image.




#### Sample Result of Analysis
```
{'Atelectasis': 0.08737277, 'Consolidation': 0.34690064, 'Infiltration': 0.5491375, 'Pneumothorax': 0.5038589, 'Edema': 0.03386995, 'Emphysema': 0.25006303, 'Fibrosis': 0.50172067, 'Effusion': 0.17096925, 'Pneumonia': 0.42648292, 'Pleural_Thickening': 0.095979005, 'Cardiomegaly': 0.15165368, 'Nodule': 0.20551245, 'Mass': 0.3035163, 'Hernia': 0.0015485223, 'Lung Lesion': 0.14447463, 'Fracture': 0.10621452, 'Lung Opacity': 0.12475821, 'Enlarged Cardiomediastinum': 0.17088078}
```