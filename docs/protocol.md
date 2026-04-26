<div align="center"><img src="images/icon_1024x1024.png" width="30%"></div>

# Undertone Protocol Document

This document lays out the protocols for moving and storing of data.

## General User Data

### User Data

```rust
bitflags! {
    struct UserState : u16 {
        const AWAY      = 0b0000_0000_0000_0001;
        const MUTED     = 0b0000_0000_0000_0010;
        const DEAFENED  = 0b0000_0000_0000_0100;
        const SILENCED  = 0b0000_0000_0000_1000;
        const DROPPED   = 0b0000_0000_0001_0000;
        const TYPING    = 0b0000_0000_0010_0000;

        // The source may set any bits.
        const _ = !0;
        }
    }
```

```rust
struct User {
    id: Uuid, // v7, used as index in db.
    email: String, // SHA256 Hashed
    password: String, // Store as hex string, bytes may not be valid UTF8
    display_name:  String, // User defined display name.
    salt: String, // Random salt generated when account created for hashing.
    created: DateTime, // UTC date time
    last_login: DateTime, // Last connection
    suspended: bool, // Whether account is suspended. If true look up the status in another table.
    roles: Vec<Uuid>, // Collection of role ids.

    // Not persisted
    permission_cache: PermissionCache, // A cache of channels and permisions for quick lookups.
    state: UserState, // Away, Muted, Deafened, Silenced, Dropped etc...
}
```

### Roles

```rust
struct Role {
    id: Uuid, // v7
    name: String,
    color: String // Hex string.
    icon: Uuid, // v5 Pointer to icon file (deterministic uuid for url/path)
    general_permissions: u32,
    voice_permissions: u32,
    text_permissions: u32,
}
```

## Channels

```rust
#[repr(u8)]
enum ChannelType {
    Unknown = 0,
    TextCategory = 1,
    TextChat = 2,
    TextForum = 3,
    TextEvent = 4,
    TextFeed = 5,
    VoiceCategory = 127,
    VoiceChat = 128,
    }
```

```rust
struct Channel {
    id: Uuid, // v7 database id
    network_id: u16,
    type: ChannelType,
    parent: &Channel,
    children: HashMap<Uuid, &Channel>
    name: String,
    title: String,
    permissions: HashMap<Uuid, u32>, // Role to Permission Bit-field mapping.
}
```

## Permissions

Permissions are stored in an u32 bit-field (32 possible flags), with multiple fields to represent different permission groupings. By nature the bit-field based RBAC permission system is additive, so rolls can only add permissions, while all permissions default to off, creating a waterfall effect.

```
 Rank   bits
 1      0101
 2      0001
 3      1000
------------
user  : 1101
```

On some occasions we need a way to block permissions such as a suspended user or temporary mute. To achieve these we are able to place blocking or deny roles into the hierarchy which then uses its bits to negate any permissions below it when set.

```
 Rank   bits
 1      0011
 2      0101 <-- Blocking role
 3      0101
 4      1000
------------
user  : 1011
```

In the above example the blocking role cancels the 3rd and 1st bits preventing role 3 from applying those permissions. But it does not block the fourth bit, allowing it to apply its permission, and role one which has priority over the blocking roll enables bits one and two. The User struct stores the calculated bit-fields for general permissions, voice permissions and text permissions. These are gained from roles and enable those permissions globally, so should be assigned very carefully to prevent abuse by users.

Per text channel and voice channel permissions store bit-fields to mark permissions per role. Generally most users roles should not enable permissions and instead roles should be granted through channel permissions as this provides much finer control. To help managing all the channel based permissions they are automatically inherited from parent channel and categories. You may set whether a permission is inherited per channel in which case it will only evaluate that bit based on its own role permissions. Also worth noting a channel may enable defaults on permissions as well to provide permissions if someone doesn't have a matching role.

To enable faster calculations all permissions are calculated and cached into single bit-fields on the user for their per channel permissions. This avoids evaluating the roles every time a permission must be checked, though any change in role or permissions on the user, role configuration or channel will invalidate that cache and trigger a recalculation. Caches also do not survive disconnect and will be recalculated on each connection to avoid mismatch for changes that occur when a user is offline.

```rust
struct PermissionCache {
    general: u32            // general permissions.
    voice: HashMap<u16,u32> // Channel Id and permissions.
    text: HashMap<u16,u32>  // Channel Id and permissions.
}
```

### Global

| Bit | Permission |
| ---: | --- |
| 1 | ROLE_CREATE |
| 2 | ROLE_DELETE |
| 3 | ROLE_MODIFY |
| 4 | ROLE_GRANT  |
| 4 | BAN |
| 5 | KICK |
| 6 | TIMEOUT |
| 7 | MOVE_USER |
| 8 | SERVER_SETTINGS_VIEW |
| 9 | SERVER_SETTINGS_MODIFY |
| 10 | SERVER_SHUTDOWN |
| 11 | SERVER_RESTART |
| 12 | SILENCE_USER |

### Voice Channel

| Bit | Permission |
| ---: | --- |
| 1 | VIEW |
| 2 | JOIN |
| 3 | TRANSMIT |
| 4 | EDIT |
| 5 | MOVE |
| 6 | MUTE_SELF |
| 7 | MUTE_OTHER |
| 8 | DEAFEN_SELF |
| 9 | DEAFEN_OTHER |
| 10 | SET_TITLE |
| 11 | GRANT_PRIORITY |
| 12 | SET_MAX_ROLL_TRANSMIT |
| 13 | MOVE_OTHER |

### Text Channel

| Bit | Permission |
| ---: | --- |
| 1 | VIEW |
| 2 | VIEW_HISTORY |
| 3 | CREATE_MESSAGE |
| 4 | CREATE_POLL |
| 5 | CREATE_EVENT |
| 6 | OWNED_MESSAGE_EDIT |
| 7 | OWNED_MESSAGE_DELETE |
| 8 | OTHER_MESSAGE_EDIT |
| 9 | OTHER_MESSAGE_DELETE |
| 10 | UNUSED |
| 11 | BLIND_USER |
| 12 | HIDE_USER |
| 13 | REPORT_MESSAGE |
| 14 | ATTACH_IMAGE |
| 15 | ATTACH_FILE |
| 16 | ATTACH_LINK |
| 17 | SET_SLOW_MODE |
| 18 | IGNORE_SLOW_MODE |
| 19 | SET_SPEAKING_ROLE_REQUIREMENT |


## Networking

Undertone is a remote server and client system for voice communication over the network. At the core of the network protocol are two categories of message types: `Ephemeral` and `Reliable`. These represent the data's need for guaranteed delivery and resistance to packet loss and corruption. A `Reliable` packet is used for things like control, chat, state updates for users on the server etc., when we want to be sure they arrive and if they don't they get resent. `Ephemeral` messages are designed to carry frequently broadcast information that we don't mind if it doesn't arrive, and if it shows up late and out of sequence we can drop it without harm to the system or user experience. Ephemeral` messages are ideal for things like voice or user position updates, that should be fast and regular.

All messages will use a common header which provides some very basic data to hint to the server and client how it should interpret the packet and handle the stream.

### Headers

```rust
#[repr(u8)]
enum HeaderTag {
    Ping = 0,
    Handshake = 1,
    Auth = 2,

    VoiceChatAudio = 10,
    WorldAudio = 11,
    SubMixAudio = 12,

    UserUpdate = 20,
    ChannelUpdate = 21,
    ChatUpdate = 22,
    RoleUpdate = 23,
    ClientCommand = 24,
    ServerCommand = 25,

    ClientPosition = 40,
    ServerPosition = 41,
}
```

```rust
bitflags! {
    struct HeaderFlags : u8 {
        const PRIORITY  = 0b0000_0001;
        const RESERVED2 = 0b0000_0010;
        const RESERVED3 = 0b0000_0100;
        const RESERVED4 = 0b0000_1000;
        const RESERVED5 = 0b0001_0000;
        const RESERVED6 = 0b0010_0000;
        const RESERVED7 = 0b0100_0000;
        const RESERVED8 = 0b1000_0000;

        }
}
```

```Rust
/// 8 byte fixed header used at the start of every message.
struct FixedHeader {
    version:     u8,    // Protocol version for compatibility checking and knowing handling rules.
    tag:         u8,    // Unique tag for the type of packet and its contents.
    flags:       u8,    // Flags
    reserved:    u8,    // One byte padding to align with words and reserve functionality.
    sequence:   u16,    // Monotic message id used for responses and tracking.
    size:       u16,    // Total size of the message in bytes including the header
}
```

```rust
struct Ping {
    id: u32, // Unique ping id for response.
    timestamp: util::TimeStamp, // u64 seconds and u32 nanoseconds converts to and from std::time::Duration.
}
```

### Audio

```rust
bitflags! {
    struct AudioFlags : u8 {
        const START     = 0b0000_0001; // Let client know this is the start of a transmission (PTT key Down)
        const END       = 0b0000_0010; // Let client know this is the end of a transmission (PTT key Up)
        const RESERVED3 = 0b0000_0100;
        const RESERVED4 = 0b0000_1000;
        const RESERVED5 = 0b0001_0000;
        const RESERVED6 = 0b0010_0000;
        const RESERVED7 = 0b0100_0000;
        const RESERVED8 = 0b1000_0000;
    }
}
```

```rust
struct VoiceChatAudio {
    user_id: u16,       // Transmitting user network id.
    channel_id: u16,    // Target channel network id.
    audio_flag: u8,     // Voice Chat flags.
    audio: Bytes,       // audio codec bytes.
}
```

```rust
struct WorldAudio {
    user_id: u16,       // Transmitting user network id.
    volume: u8,         // Volume Enumerator ("whisper", "Talk", "Shout" for RP and range purposes.)
    audio_flags: u8,    // Audio chat flags
    audio: Bytes,       // raw audio codec bytes.
}
```

```rust
struct SubMixAudio {
    user_id: u16,       // Transmitting user network id.
    channel_id: u16,    // SubMix id for subscription notification.
    kind: u8,           // SubMix kind id.
    strength: u16,      // strength
    interference: u16,  // interference value
    audio: Bytes,       // raw audio codec bytes.
}
```

