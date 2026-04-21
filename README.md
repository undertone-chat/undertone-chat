# Undertone

In the world of online roleplay there are few free and flexible options for realtime communication, a critical part of the roleplay experience. Most solutions are either expensive to license or are very specific to one particular genre or game. Undertone solves these issues by providing an open source and free to use real time voice communication solution which is:
1. Easily extensible through scripting to work with any game or genre.
2. Provides out of the box solutions for the most common roleplay games and requirements.
3. Built on rust to be fast and platform agnostic.
4. Scallable server and client that can work on even bad connections and slow computers to remain inclusive of all communities.

## Values
It is difficult in this day and age to make promisses or commitments when technology and soci-econmic factors change so rapidly.  While I would love to make a guarentee or promise about a policy and stand by it for ever, it would be naive to do so knowing how important context when evaluating decisions about a product.  To that end I do want to set forth some values which can be used to guide the development and future decisions of Undertone.

### Privacy First
Your right to privacy will always be a top priority for the developers and every reasonable effort will be made to ensure your personal information is never leaked or exposed.

### Ethical Finances
Communities are the foundation of successful projects, and the community that Undertone is intended to serve often has to do a lot with very little, so you will never be charged to use the `Undertone Client` or `Undertone CLI Server` regardless of user counts so long as your community is not charging for access or play. For users or communities that charge for their services, they may license Undertone for reasonable flat fees.

Finally **ANY** community that publically supports hate, discrimination or harm against any persons may have their access revoked.

### Open Access
Undertone uses a copy left AGPL 3.0 [LICENSE](LICENSE) to ensure the product can remain free and any variations or forks of the project will carry that license forward. The intent is that what ever has been learned and created during the development of this product was done on the backs of giants using software and knowledge that has been freely shared and distributed without bias.  Therefor the product of those things should also remain freely available to use and learn from.

### Do No Harm
At the time of writing this, the idea of Do No Harm as a developer and company was front and center in the zeitgeist. It is a fundamental beleif that as the creators of ideas and products it is on us to make choices in how our products and resources may be used to help prevent harm to others, regardless of if they use the products or not. In the case of community based software this is especially important as the free and privacy first nature of Undertone may be very appealing to those who actively harm others. To combat those individuals several tools will be put into place to control the ability to use the software for harmm without exposing normal users data or privacy.

## Features of Undertone
This is a non-exhaustive list of features that we hope will be present by the 1.0 release of Undertone. We will not  be attaching dates or version targets for the features to ensure development can proceed in a way that is healthy and
 organic without artifical deadlines.

- [ ] *End to end encryption* (E2E): Everything from your voice to login data and chat will be encrypted E2E, even we wont know what you are saying!
- [ ] *Hierarchal Role Based Access Control* (RBAC) for users.
- [ ] *Voice Chat*: This is the core of the project and its main intended use.
    - [ ] *Channels* - Server Admins may create channels for users to chat in using voice or text messaging.
    - [ ] *Submix* - Communication with users outside of channels using subscription based packet routing and policies. These will be the backbone of things like radios, whispers or even supenatural communications.
    - [ ] Positional - Allow mixing sources in channels and sub mixes so they appear to emit from 3D space around the listener by attaching coordinates to the voice packet.
    - [ ] Effects - Allow data driven effect chains to change EQ, reverb and other audio effects on a per source basis.
- [ ] Cross Plaform Desktop Client - Built on Dioxus, rust and modern CSS.
- [ ] Cross Plaform CLI Server - Built on rust cause its damn cool.
- [ ] Server Manager - Web based server management tool.
- [ ] Text based chat channels.
- [ ] Forum / Thread channels.
- [ ] Authorization - Provide optional authorization tools to control user access and privledge
    - [ ] OAuth - Allow users to register on your server using OAuth providers like other voice softwares that shall remain nameless.
    - [ ] Email & Password - A very simple auth system to allow for basic login and account control using a users email and password as authentication tokens.
    - [ ] 2FA - Enable two factor authentication for extra security and to help users protect their accounts.
- [ ] *Lua Scripting* - MLUA with LuaJIT compilation for fast, familiar and reliable extension. This should enable polling of games and other API's to provide data for Undertone's data driven voice pipeline.
- [ ] *Data API* - `Undertone Client` exposes a slim API to allow games to be modded and transmit information to the client which may be encoded.
- [ ] Persistent storage options:
    - [ ] *Key Value Pair* (Plain Text) - The simplest form of database using a plain text flat file to store user and server data.  Can become slow and cumbersome quickly, but good for testing and base functionality.
    - [ ] *Key Value Pair* (Binary) - using binary KVP storage and efficient libraries the file remains local to the server but is in encrypted.
    - [ ] Database Adapter - Adapters for common databases:
        - [ ] `MongoDB` A very popular NoSQL document storage database.
        - [ ] `PostgreSQL` Cornerstone of relational data.
        - [ ] `MySql` / `MariaDb` The ultimate opensource DB.

## Project Dependencies

Every great project is built on the backs of giants who came before it. Undertone is no different, though we strive to minimize the amount of dependencies to keep the project sleek and light weight, it would be irresponsible to re-invent every wheel we need when solutions already exist.  This list is non-exhaustive and can and will change over the lifetime of the project. For version specific dependencies be sure to check the CHANGELOG.

### Global

Dependencies called globally between crates and applications.

- [rkyv](https://crates.io/crates/rkyv) - for serialization of data structures across the project with blazing fast zero copy access.
- [rkyv-dyn](https://crates.io/crates/rkyv-dyn) - allow trait serialization.
- [flyweights](https://crates.io/crates/flyweights) - Interning string allocations for user strings.

### Crates

Crate only dependencies that may also represent sub-level separation not noted here when crate specific.

- [thiserror](https://crates.io/crates/thiserror) - macros to assist in effective error type creations.
- chrono - Time keeping with rkyv compatible  Archive shapes for transmission and storage.
### Application

- [anyhow](https://crates.io/crates/anyhow) - Easier error handling in application environments.
- [tokio](https://tokio.rs/) - Async Networking and thread handling.
### Client

- [Dioxus](https://dioxuslabs.com/) - Rust centric cross platform framework that leverages the power of webview (or native eventual) rendering.  Tightly coupled to rust it allows us to keep our focus in one primary language and share code more easily.

### Server

*TBD*

## Networking

Undertone uses two primary network layers to accomplish fast and reliable communication between the users.

### Client <-> Server Architecture

Due to the nature of data being transmitted all clients will pass their communications through a central server, hosted by the community owner(s) on their own hardware or leased through a provider.  It is recommended that the server be run with or adjacent to the game server with shared network to allow for more complex interactions via scripting extensions and server authorative interactions.

#### Control Layer

To handle any messaging between clients and server that must be reliable a TCP/IP based control layer enables reliable communications with guaranteed delivery.  This is achieved at the cost of speed and processing power so this layer should never be used for time sensitive or high volume traffic.

#### Voice & Data Layer

To enable high speed low overhead transmission of time critical but unreliable information Undertone uses a UDP layer which transmits voice packets that can contain meta data about the user's state while transmitting such as 3D position, effect parameterization and info about side traffic (radio or other non channel based communications)

### Control Protocol

#### CommandAck

Used when a command requires specific acknowledgement with potential additional data from either the server or client.

```rust
struct CommandAck {
status: CommandStatus,
command_id: u32,
command_type: CommandType,
}
```

#### KeepAlive

Ping from the server to a client to keep the connection open and ensure client is healthy.  Expected response is `CommandAck`.

```rust
struct KeepAlive { }
```

#### ClientInfo

A data shape containing information about a client and their status. Typically used when client connects, their status changes or information is requested by another client.

```rust
struct ClientInfo {
    display_name: String,
    connected_at: TimeStamp, // Seconds since midnight UTC
    }
```
