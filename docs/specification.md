# Undertone Software Requirements Specification
<div align="center">
<img src="icon_1024x1024.png" width="50%">
</div>

***Real-time voice and text and communication software for role-play and sim communities.***  

| Field | Information |
|---:|---|
| Title | Software Requirements Specification for Undertone |
| Date | April 24th, 2026 |
|Last Updated | April 24th 2026|
| Status | Draft |


***Glossary***
| Term | Definition |
| ---: | :--- |
| AEC| Accoustic Echo Cancellation |
| AI | Artifical Intelligence |
| API | Application Programming Interface |
| APPI | Act on the Protection of Personal Information (Japan) |
| CNG | Comfort Noise Generation |
| Codec | Software component that compresses large audio or video data |
| CRC | Cyclic Redundancy Check |
| E2EE | End To End Encryption |
| FEC | Forward Error Correction
| GDPR | General Data Protection Regulation (EU) |
| LGPD | Brazillian data processing and collection laws |
| MILSIM | Military Simulation roleplay |
| Opus | A popular open source audio codec for real time communication |
| PIPA | Personal Information Protection Act (South Korea) |
| PTT | Push To Talk for voice transmision |
| QUIC | Quick UDP Internet Connection |
| RBAC | Role Based Access Control |
| RP | Role-Play or roleplaying |
| Rust | Cross-Platform programming language |
| Sim | A game or simulator that strives for realism |
| SRS | System Requirements Specification |
| TCP/IP | Transmission Control Protocol / Internet Protocol |
| TLS | Transport Layer Security |
| TOS | Terms Of Service |
| UDP | User Datagram Protocol |
| VAT | Voice Activated Transmission |

## 1. Introduction

### 1.1 Purpose
This Software Requirement Specification (SRS) document provides a description of the real time communication software Undertone being developed by [Cephy314](https://github.com/Cephy314). This document details the functional and nonfunctional requirements for the software, which aims to create a community controlled and focused product to allow a single product solution that is free to use.

This SRS will serve as the foundation for the subsequent system design and development phases, ensuring that all stakeholders have a clear understanding of what the software will do and how it will operate.

### 1.2 Document Conventions

This document follows these conventions:
| Term | Definition |
| --- | :--- |
| ***SHALL*** | Refers to mandatory requirement that must be fulfilled during Phase 1 of development. The vendor / developer is required to cover this feature in the current implementation phase. |
| ***SHOULD*** | Indicates a requirement that will take place in phase 2 and the developer should take it into consideration for future scalability. |
| ***MAY*** | Refers to a requirement anticipated for phase 3 or subsequent phases. The developer is encouraged to consider this requirement generally, keeping future scalability in mind. |
| ***TBD*** | To Be Determined, indicates information that is not yet available but will be provided in future versions. |
| ***Note*** | Provides additional information or clarification. |

<br>

Requirements are categorized as follows:
| Requirement Number | Description |
| --- | --- |
| FR-XXX | Functional Requirements |
| NFR-XXX | Non-Functional Requirements |
| IR-XXX | Interface Requirements |
| DR-XXX | Data Requirements |
| SR-XXX | Security Requirements |

### 1.3 Intended Audience

This document is intended for the following stakeholders:
| Stakeholder | Role |
| --- | --- |
| Community | Users of the product that can provide feedback, requests and help evaluate success. |
| Developer | Who will design and implement the software. |
| Contributors | Community members who may wish to submit contributions and fixes |
| Derivative Developers | Those modifying their own versions of the software for internal/private use.

### 1.4 Project Scope

Undertone is made up of several peices of software and services that are run independtly but allow interaction and various forms of interaction through multiple vectors.

While intended to be operated by individual communities with their own resources allowing them to maintain control, there will be some company level integrations to link resources and provide licensing for servers and potential server hosting solutions.

#### *In Scope*

The project will include the following key components:
- **Undertone CLI Server**: Community hosted server that runs from the command line on any supported platform that acts as the single source of truth and backbone for the community.
- **Undertone Desktop Client**: Cross-Platform software to allow community members to connect to the server and interact through the different methods of communication including voice and text.
- **Shared Serialization Crate**: A rust crate to be shared across platforms to encode and decode information on different services and software interfaces.
- **Undertone Data API**: API for servers and clients to allow external applications such as games or web-services to interact and provide data to support community activities.
- **Undertone User Registration and Authentication**: Controlled global user registration to enable privacy and data collection law adherance in all regions, protecting end users from having data sold or collected. All servers, self-hosted and cloud based will interface with the same auth API.
- **LUA Scripting Language**: Allow communities to extend and creating interfaces for clients and servers to meet their specific needs.
- **Database Adapter System**: Allow servers to interface with different databases using user definable database adapters.
- **Health Monitoring Endpoint**: Accessible endpoint to allow systems to monitor server health.
- **Multilingual Support**: Localization support that allows community contributed localization definitions for system text.
- **License Key System**: Licensing for server operators (no charge) to allow control over whether servers may continue to run if in violation of TOS or License.
- **Overlay/Window System for SubMix Interaction**: Present unique visual interfaces for SubMix's to enable visual radio, phone interfaces or what ever the community can imagine.

#### *Out of Scope*
- **Undertone Cloud**: Premium hosted cloud servers and data storage for communities that lack technical expertise or resources to host their own servers, or simply wish to pay for convenience, security and reliability.
- **Server Dashboard**: Web based front end for server admins to modify and configure servers remotely.
- **Undertone Cloud Client**: Allow users to access Undertone Cloud using web based client.
- **Undertone Mobile**: Mobile client for access of any Undertone Server (Cloud or CLI)
- **Support Infrastructure**: Website that offers community based support for free users and premium support options for Cloud users.  Bounty system for support to earn Cloud credits for hosting.
- **3D Accoustic Mapping**: Custom format to store information about the world such as surfaces, and terrain to allow for accurate accoustic propogation when using the World 3D audio configurations. Game mods and tools to map the environments of popular games through either model import or user collection via traversal and realtime sampling.
- **Undertone Performer Protocol**: Advanced support for low latency/high  priority transmission and synchronization to allow for live performance over voice.
- **Server Theme Management**: Control the visual style of the client per server (Opt in)
- **Custom Client Interface Themes**: Allow users to create custom themes for their client visuals. 

### 1.5 References
1. Community Based Experience with MILSIM and RP organizations.
2. Research on realtime audio communication systems.

## 2. Overall Description

### 2.1 Product Perspective
Undertone is a new voice and text communication system being developed to create a centralized community tool for gaming communities with a focus on functionality to support RP and MILSIM needs. There is a lack of quality free options for these types of communities with high fragmentation of services across multiple platforms.

#### The platform will interface with:
- Existing game and social media platforms allowing users to engage in ritch presense and social sharing.

### 2.2 Product Functions
Undertone will provide the following core functions:
1. User Registration and Profile Management
    - Authorize for servers and linked RBAC permissions.
    - Account enabling, disabling, deletion and recovery.
    - Server administration accounts with license key association.
    - Global banning and control of accounts in violation of Undertone EULA and TOS.
2. Voice Communication
    - Low latency real time voice communication.
    - Positional voice rendering for roleplay communities and sim games (e.g. FiveM / RedM / Arma )
    - Channel based audio chat.
    - Submix system for radio, supernatural communication or sound effects driven by server and client.
3. Text Communication
    - Heirarchal RBAC and Channel/Category based permissions
    - Support for normal text chats with markdown, image and emoji support.
    - Reactions and other interactions to individual posts.
    - Announcement system for events, news, rss feeds with special formatting templates.
    - Forum style text systems for less ephemeral communication using posts and thread model.
4. Server Customization
    - Admin controlled configurations for all roles, users permissions, channels and voice systems.
    - Configuration of SubMix's with effect and other controls
    - Configuration for positional audio with unit scales and relevance filtering.
5. Server Administration
    - Allow admins to ban, mute, kick  and administer users as necessary for their community.
    - Allow List and Deny List options for regions
    - RBAC access control to shut down servers or restrict access to only certain users.
6. Database Adapters
    - Allow user choice for common databases:
        * MongoDB (NoSQL)
        * PostgreSQL
        * MySql Compatible Databases (MariaDB)
        * Disk Based KVP or SqlLite
7. Security & Privacy
    - All network communication encrytped using TLS.
    - User data secured on Undertone owned servers and fully encrypted
    - Encrypted private chats (stored per server using rotating keys) to ensure server owners cannot peek and only the users whom participated can read the data.
    - Adherence to regional data privacy laws with local storage per required region for all personal user data.

### 2.3 User Classes, Characteristics and Needs

The platform will serve the following user classes:

#### 1. Community Creators and Developers

- Creative or Technically adept users whom wish to create and host their own communities and or seeking voice solutions for their private game servers.
- Varying technical skill sets.
- Primary Needs:  Community communication hub and support for high quality roleplay and milsim functionality.

#### 2. Community Members

- End users who participate in gaming communities that are built on Undertone.
- Varying technical skill sets.
- Primary needs: Reliable voice and text communication that is bandwidth friendly with low system requirements. Even on high end computers users prioritize performance for games and want minimal resource usage for other applications.

### 2.4 Operating Environment

Undertone will operate in the following environment:

#### 1. Technical Environment

- Desktop client application running on Windows, MacOS, or Linux.
- Commandline shells for server operation on Windows, MacOS, Linux.
- Should Docker/Kubernetes for Undertone Cloud Servers.
- Web interface for Undertone global and cloud services.
- Should support Mobile options like Android and iOS.
- Should support web based client for access to Undertone Cloud servers.

#### 2. Hardware Environments

- VPS/Dedicated Servers for CLI Server, Shall function on average user owned hardware as well.
- Adequate Storage for Chat and File data.
- Network infrastructure with adequate bandwidth for large simultaneous real time voice clusters.
- Disaster Recovery Capabilities.

#### 3. Software Environment

- Windows, MacOS and Linux.
- Database Hosts / Management systems with high performance and scalability for global services.
- Local database hostings for CLI servers.
- Modern web technologies and frameworks.
- Security software for data protection and user authentication.

#### 4. User Environment

- Various devices including desktop computers and laptops.
- Should support tablets and smartphones for mobile client.
- Varying internet connection speeds and reliability.
- Multilingual user interface (localization)
- Accessibility support for users with disabilties.

### 2.5 Design and Implementation Constraints

The following constraints will implact the design and implementation of the platform:

#### 1. Technical Constraints

- Must be usable by users on multiple operating systems of varying age.
- Must operate on older hardware and laptops.
- Global services must be accessible via standard web browsers without special plugins or setings.
- Must function effectively for users with low bandwidth or moderate latency connections.
- Must be user friendly with accessibility in mind.

#### 2. Regulatory Constraints

- Must comply with regional data privacy and protection regulations
- Must adhere to international standards for data security (e.g. GDPR principles)
- Must support legal requirements for age verification on servers with adult content.

#### 3. Business Constraints

- Core product must be freely available to users.
- Data must ***never*** be sold.
- Must scale quickly to accomodate shifting user acceptance and engagement
- Must be implemented in phases using MVP principles.

#### 4. User Constraints

- Must accomodate users with varying levels of technical proficiency.
- Must support accessibility standards for users with disabilities.
- Must provide multi-lingual support for community submitted localizations.

### 2.6 User Documentation

The following user documentation will be developed as part of the system:

#### 1. Online Help System
- Context sensitive help for all system functions
- Frequently Asked Questions (FAQs)

#### 2. User Manuals
- Server setup guide
- User registration and profile configuration guide
- Server operation guide
- Server customization guide

#### 3. Training Materials
- Quick reference guides
- System demonstration materials (videos, slides with screenshots and instructions).
- Example code for configuration and customization

### 2.7 Assumptions and Dependencies

#### Assumptions

1. Community admin will provide individual support to their communities.
2. Community admin will provide their own hosting, database and backups for CLI servers.
3. Adequate infrastructure will be available for system deployment of global services
4. Stakeholders will participate actively in requirements validation process.
5. Products will be implemented in phases with basic functionality prioritized.

#### Dependencies

1. Community support for development and testing
2. Additional voulenteer developers to get product intial launch complete.
3. Financial stability for global services hosting and continued development.

## 3. System Features and Functional Requirements

This section details the functional requirements of Undertone, organizaed by major system features.  Each requirement is identified with a unique ID (FR-XX) for tracability and reference.

### 3.1 User Management and Authentication

#### 3.1.1 User Registration and Profile Management

| ID⠀⠀⠀ | Requirement|
| --- | --- |
| FR-001 | The dystem ***SHALL*** provide a self-registration process for users. |
| FR-002 | The system ***SHALL*** support account activation by user mobile number. |
| FR-003 | The system ***SHALL*** collect the following information from users:<br>**Required**: Email, Identifier, Age, Password, Recovery Codes/Words<br>**Optional**: Mobile, Social Links, Info, Pronouns |
| FR-004 | The system ***SHALL*** support uploading of Profile Pictures (PFP) |
| FR-005 | The system ***SHALL*** provide generic or default profile picture options |
| FR-006 | The system ***SHALL*** enable users to set up 2FA. |
| FR-007 | The system ***SHALL*** allow login via mobile six digit code |
| FR-008 | The system ***SHALL*** allow account recovery via email<br>or mobile with user defined recovery code words. |
| FR-009 | The system ***SHALL*** allow users to set privacy options to hide<br>any potentially public information or profile |
| FR-010 | The system ***SHALL*** provide public url for non-hidden profiles. |
| FR-019 | The system ***SHALL*** Use unique hashing to prevent users from registering multiple accounts to the same email and or mobile without exposing user data. |

#### 3.1.2 Community Registration and Management

| ID⠀⠀⠀⠀ | Requirement |
| ------ | ----------- |
| FR-011 | The system ***SHALL*** allow users to register up to one communities and provide a license key on success. |
| FR-012 | The system ***SHALL*** allow the user to upload icon and profile images for their community. |
| FR-013 | The system ***SHALL*** allow the user to customize the community with descriptions and searchable content. |
| FR-014 | The system ***SHALL*** allow inclusion of links to videos for the server. |
| FR-015 | The system ***SHALL*** allow the user to set up the IP and port of the server for status verification |

#### 3.1.3 User Administration

| ID⠀⠀⠀⠀ | Requirement |
| ------ | ----------- |
| FR-016 | The system ***SHALL*** allow system adminstrators to suspend, ban and restrict user accounts. |
| FR-017 | The system ***SHALL*** allow administrators to send users messages via email for adminstrative actions |
| FR-018 | The system ***SHALL*** generate administrator tickets from users email responses when sent from valid email address unique associated key. |
| FR-020 | The system ***SHALL*** maintain an audit log for all administrative actions. |
| FR-021 | The system ***SHALL*** maintain logs of email histories to and from users. |

#### 3.1.4 Authentication and Authorization

| ID⠀⠀⠀⠀ | Requirement |
| ------ | ----------- |
| FR-022 | The system ***SHALL*** implement secure authentication mechanisms including username/password, email verification, and multi-factor authentication options. |
| FR-023 | The system ***SHALL*** enforce strong password policies with configurable parameters |
| FR-024 | The system ***SHALL*** implement role-based access control for community administration by multiple users. |
| FR-025 | The system ***SHALL*** provide session management with configurabel  timeout settings. |
| FR-026 | The system ***SHALL*** maintain detailed access logs for security monitoring and auditing. |

### 3.2 Voice Communication

#### 3.2.1 Core Voice Features
| ID⠀⠀⠀⠀ | Requirement |
| ------ | ----------- |
| FR-027 | The client ***SHALL*** allow users to select recording and playback hardware. |
| FR-028 | The client ***SHALL*** allow users to preview their sound devices to confirm functionality |
| FR-029 | The client ***SHALL*** support automatic gain control. |
| FR-030 | The client ***SHALL*** support noise cancellation |
| FR-031 | The client and server ***SHALL*** support Opus audio codec |
| FR-032 | The client ***SHALL*** support automatic coded tuning based on user preference and connection speed and latency |
| FR-033 | The client ***SHALL*** support voice activated transmission |
| FR-034 | The client ***SHALL*** support push to talk transmission |
| FR-035 | The client and server ***SHOULD*** support newer AI audio codecs.|
| FR-036 | The client ***SHALL*** support spatial audio mixing. |
| FR-037 | The client ***SHALL*** support effect chains in audio mixing |
| FR-038 | The client ***SHALL*** support adaptive jitter buffer |
| FR-039 | The client ***SHALL*** support comfort noise generation (CNG) |
| FR-040 | The client ***SHALL*** support packet loss concealment via forward error correction (FEC) |
| FR-041 | The client ***SHALL*** support audio packet re-ordering |
| FR-042 | The client ***SHALL*** support stale packet filtering |
| FR-043 | The client ***SHALL*** support Accoustic Echo Cancellation |
| FR-044 | The client and server ***SHALL*** support quality of service (QOS) tags (DSCP) via Differentiated Services (DiffServ) to market packets as high priority (e.g. EF - Expedited Forwarding) |
| FR_057 | The client ***SHOULD*** support auto ducking of system audio when receiving audio |
| FR-058 | The client ***SHOULD*** allow users to be able to adjust the ducking threshold and level |
| FR-059 | The client ***SHOULD*** support ducking of audio when user is transmitting. |

#### 3.2.2 Channel Based Voice

| ID⠀⠀⠀⠀ | Requirement |
| ------ | ----------- |
| FR-045 | Server ***SHALL*** support creating voice channels |
| FR-046 | Server ***SHALL*** support heirarchal channel configuration with inherentence. |
| FR-047 | Server ***SHALL*** support per channel RBAC | 
| FR-048 | Users ***SHALL*** be able to adjust volume of individual speakers locally |
| FR-049 | Users ***SHALL*** be able to mute individual speakers locally |
| FR-050 | Users ***SHALL*** be able to mute them selves in channel. |
| FR-051 | Users ***SHALL*** be able to deafen them selves in channel. |
| FR-052 | Users ***SHALL*** be able to mark them selves as away while in channel. |
| FR-053 | Users ***SHALL*** be able to move between channels they have access to. |
| FR-054 | Users ***SHALL*** be able to configure audio notifications for users joining and leaving channels. |
| FR-055 | Users ***SHALL*** **NOT** be able to use Channel and World voice at the same time.
| FR-056 | Channels ***SHALL*** support priority speakers being marked by users with correct permissions, enabling ducking of other voices when the speaker is transmitting. |
| FR-060 | Users ***SHALL*** be able to drag them selves between channels in the channel list. |
| FR-061 | Users ***SHALL*** be able to drag other users between channels if they have sufficient permissions. | 