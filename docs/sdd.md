![Arrow Banner](https://github.com/Arrow-air/tf-github/raw/main/src/templates/doc-banner-services.png)

# Software Design Document (SDD) - `svc-docs`

## :telescope: Overview

This document details the software implementation of docs.

This service is responsible for docs

### Metadata

| Attribute     | Description                                                       |
| ------------- |-------------------------------------------------------------------|
| Maintainer(s) | [Services Team](https://github.com/orgs/Arrow-air/teams/services) |
| Stuckee       |  |
| Status        | Draft                                                             |

## :books: Related Documents

Document | Description
--- | ---
[High-Level Concept of Operations (CONOPS)](https://github.com/Arrow-air/se-services/blob/develop/docs/conops.md) | Overview of Arrow microservices.
[High-Level Interface Control Document (ICD)](https://github.com/Arrow-air/se-services/blob/develop/docs/icd.md)  | Interfaces and frameworks common to all Arrow microservices.
[Requirements - `svc-docs`](docs - generate a link specifically for this module's view in NocoDB) | Requirements and user stories for this microservice.
[Concept of Operations - `svc-docs`](./conops.md) | Defines the motivation and duties of this microservice.
[Interface Control Document (ICD) - `svc-docs`](./icd.md) | Defines the inputs and outputs of this microservice.

## :dna: Module Attributes

| Attribute       | Applies | Explanation                                                             |
| --------------- | ------- | ----------------------------------------------------------------------- |
| Safety Critical | Yes/No  | |
| Realtime        | Yes/No  | |

## :globe_with_meridians: Global Variables

**Statically Allocated Queues**

docs

## :gear: Logic

### Initialization

docs Description of activities at init

### Control Loop

docs Description of activities during loop
As a GRPC server, this service awaits requests and executes handlers.

All handlers **require** the following environment variables to be set:
- docs

For detailed sequence diagrams regarding request handlers, see [REST
Handlers](#mailbox-rest-handlers).
For detailed sequence diagrams regarding request handlers, see [gRPC
Handlers](#speech_balloon-grpc-handlers).

### Cleanup

docs Description of activities at cleanup

## :mailbox: REST Handlers

docs flowcharts for rest handlers

## :speech_balloon: gRPC Handlers

docs flowcharts for gRPC handlers
