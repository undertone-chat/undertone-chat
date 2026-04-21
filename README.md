# Undertone

In the world of online roleplay there are few free and flexible options for realtime communication, a critical part of the roleplay experience. Most solutions are either expensive to license or are very specific to one particular genre or game. Undertone solves these issues by providing an open source and free to use real time voice communication solution which is:
1. Easily extensible through scripting to work with any game or genre.
2. Provides out of the box solutions for the most common roleplay games and requirements.
3. Built on rust to be fast and platform agnostic.
4. Scallable server and client that can work on even bad connections and slow computers to remain inclusive of all communities.

Undertone is a community tool, built and maintained by the community and will always be free and available. Developers are welcome to fork and create their own branded or specific versions as long as they contribute changes back to the main project and remain open source and free to use as per the [LICENSE](LICENSE).  The only exception will be for companies hosting servers as a service will have to purchase licenses from the organization to help fund continued development and ensure a healthy ecosystem for the community.

## Project Structure



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
