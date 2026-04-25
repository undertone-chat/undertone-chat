

# Undertone - Software Requirement Specification

Real-time voice and text and communication software for role-play and sim communities.

| Field | Information |
|---:|---|
| Title | Software Requirements Specification for Undertone |
| Date | April 24th, 2026 |
|Last Updated | April 24th 2026|
| Status | Draft |


***Glossary***
| Term | Definition |
| ---: | :--- |
| AI | Artifical Intelligence |
| API | Application Programming Interface |
| Codec | Software component that compresses large audio or video data |
| CRC | Cyclic Redundancy Check |
| E2EE | End To End Encryption |
| GDPR | General Data Protection Regulation |
| MILSIM | Military Simulation roleplay |
| Opus | A popular open source audio codec for real time communication |
| QUIC | Quick UDP Internet Connection |
| RBAC | Role Based Access Control |
| RP | Role-Play or roleplaying |
| Rust | Cross-Platform programming language |
| Sim | A game or simulator that strives for realism |
| SRS | System Requirements Specification |
| TCP/IP | Transmission Control Protocol / Internet Protocol |
| TLS | Transport Layer Security |
| UDP | User Datagram Protocol |

## 1. Introduction

### *1.1 Purpose*
This Software Requirement Specification (SRS) document provides a description of the real time communication software Undertone being developed by [Cephy314](https://github.com/Cephy314). This document details the functional and nonfunctional requirements for the software, which aims to create a community controlled and focused product to allow a single product solution that is free to use.

This SRS will serve as the foundation for the subsequent system design and development phases, ensuring that all stakeholders have a clear understanding of what the software will do and how it will operate.

### *1.2 Document Conventions*

This document follows these conventions:
| Term | Definition |
| --- | :--- |
| ***SHALL*** | Refers to mandatory requirement that must be fulfilled during Phase 1 of development. The vendor / developer is required to cover this feature in the current implementation phase. |
| ***SHOULD*** | Indicates a requirement that will take place in phase 2 and the developer should take it into consideration for future scalability. |
| ***MAY*** | Refers to a requirement anticipated for phase 3 or subsequent phases. The developer is encouraged to consider this requirement generally, keeping future scalability in mind. |
| ***TBD*** | To Be Determined, indicates information that is not yet available but will be provided in future versions. |
| ***Note*** | Provides additional information or clarification. |

