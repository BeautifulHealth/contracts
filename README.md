## Beautiful Health: Decentralised Patient-Centric Health Care Record and Decentralised AI Diagnostic Platform

### Abstract
Beautiful Health is a decentralised healthcare record system designed for the Multiverse X hackathon #ProofOfHack. It prioritises patient data privacy and control by leveraging an openEHR compliant framework built with Rust contracts on the Multiverse X blockchain. The system features an API gateway service that connects to a FastAPI Microservice running an AI diagnostic model capable of analysing chest radiographs. Beautiful Health aims to solve interoperability challenges, enhance data privacy, and provide seamless integration of advanced diagnostics, empowering patients and healthcare providers alike.

### Useful Links
1. Contract Address: erd1qqqqqqqqqqqqqpgq7d50l7gtdcp79hw2atjf42cem7up02avt73ssffcwt 
2. Devnet Explorer: https://devnet-explorer.multiversx.com/accounts/erd1qqqqqqqqqqqqqpgq7d50l7gtdcp79hw2atjf42cem7up02avt73ssffcwt



### Introduction

#### Background
In the current healthcare landscape, data management is plagued by centralised systems that compromise patient privacy and control. These systems often face interoperability issues, making it difficult for healthcare providers to access and share patient information seamlessly.

![Architecture](architecture.png)

#### Purpose
Beautiful Health addresses these issues by offering a decentralised patient-centric approach to healthcare data management. By decentralising records, patients regain control over their data, and interoperability is enhanced through standardised frameworks. Additionally, the integration of an AI model running on FastAPI ensures that advanced diagnostic capabilities are both secure and scalable. This innovative approach empowers patients and equips healthcare providers with powerful tools for improved diagnosis and treatment.

#### Scope
This ReadMe details the technical architecture and components of Beautiful Health, highlighting its innovative use of blockchain technology, API integration, and AI diagnostics.

### Problem Statement
1. Healthcare Data Privacy
Centralised healthcare systems store patient data in silos, increasing the risk of breaches and unauthorised access. Patients have limited control over their data, raising concerns about privacy and security.

2. Interoperability Challenges
Different healthcare providers use various systems and standards, creating significant barriers to effective data exchange. This fragmentation leads to inefficiencies and potential errors in patient care.

3. Patient Control
Traditional systems often do not provide patients with easy access to their own health records. Patients cannot easily share their data with providers or manage permissions for data access, leading to a lack of empowerment.