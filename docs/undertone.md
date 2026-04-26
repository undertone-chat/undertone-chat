<div align="center"><img src="images/icon_1024x1024.png" width="30%"></div>

# Undertone
Undertone is a real time voice and chat solution designed with roleplay and gaming communities as its users. The specialized needs of various sim, roleplay, gaming communities are not properly met in any single application, and often require significant financial investment on the part of the community to enable the better solutions through plugins or licenses. Undertone is an open source community focused solution that enables the users to decide how to invest in their solutions via development using rust and OS/Hardware agnostic designs.

## Concepts
Undertone has several modes of operation supporting various community needs through server defined configurations and administration.

### Data Persistence
Communities require data persistence, but where and how that data is stored should be controlled by the community and flexible. Undertone uses adapters to implement different forms of storage, whether local flat file style using KVP or on popular modern databases (local or remote) such as MongoDB, PostgreSQL or MySQL and compatible variants. Because of the value of the communities data, adapters must also support methods for backup and restoration of data, but ultimately it will rely on the community to handle their backups and security.

### Authentication and Permissions
Every Undertone server requires some form of authentication for users, whether that is a custom auth adapter developed by the community to interface with their own existing solutions or using popular methods like OAuth (Authenticate through other services like Discord, Twitch, Steam, Google etc) or even simple local user name and password or email and access token methods.  In the end these options are up to the communities that use Undertone while the developer team will do its best to make sure the most common and popular adapters are available and maintained.

For permissions every account will also use Hierarchal Role Based Access Control (RBAC) to determine what, where and how they can do things on the server. Whether it is the Admin role which allows world domination or a Guest role that just allows someone to view one chat channel without posting.  This methodology allows for inheritence in permissions to make configuring roles simpler and more intuitive and ultimate flexibility for setting up your servers the way you want.

### Encryption and Privacy
It is the goal of Undertone to support End To End Encryption (E2EE) encryption using cryptographic solutions such as TLS and 128bit encryption of private user data. This way if the community ever suffers a breach of data, personal user information will always be encrypted and protected.  Undertone at no time will ever ask for or store any financial information like credit cards or bank information.

#### Voice Encryption
All voice transmissions are encrypted with TLS to prevent eavesdropping or capturing of conversations on the wire.

#### Private Chat Encryption
Private chats between community members (think DM's) are encrypted using the public keys of the different group members.  When someone enters or leaves the chat the encryption key is adjusted so that they cannot decrypt newer messages.  This will effectively prevent new users to a private group chat from seeing previous messages as well but ensures no one outside of those present when a message was sent will ever be able to read it.  While our primary goal is to give community control over how their social servers are handled, we also acknowledge users need for privacy even in a trusted community.

### Voice Communication
Undertone has three primary methods of voice chat which can be used to solve solutions needed by different roleplay, sim or social communities. Channels and World audio modes are mutually exclusive to avoid confusion, complication, and potential for cheating or subverting intent. If a user is in a voice channel they will not have access to the 3D positional audio from the game/world. If they are in World mode they cannot access channels. SubMix will work in either mode allowing servers and communities work in their own way whether they prefer channel or world based communication.

#### Channels
This should be familiar to anyone who has ever used popular voice chat products such as Discord, TeamSpeak or Mumble. Server configured voice channels that allow users to join in live voice chat conversations. Channels support hierarchical configuration and will honor user permissions. For voice features users will be able to mute or adjust the volume they hear specific users at.

#### World
World mode is for allowing users to communicate in 3D positional audio where they hear those around them in spatial rendering.

#### SubMix
This is our magic subscription based audio communication system that will allow servers to set up and configure things like Radios, Sound Effects, even supernatural abilities like telepathy. Using SubMix the server owners can configure them to operate how they desire and even tune the effects and ways they are influenced. Submix rely on values like strength, quality, and other tuneable variables instead of 3d position.

### Text Communication
Undertone will also provide text chat communication tools to provide a common solution for communities that allows them to use one single software solution if they choose to.  There is absolutely no requirement to use these features and established community tools like Discord can still be used and even integrated using Undertone's discord bot for role synchronization when using Discord as an OAuth provider.

#### Chat Channels
Your typical chat channels that support Markdown rendering with options like code blocks, headings, even LaTeX and Mermaid chart rendering. Communities will be able to configure the markdown support options based on their needs and desires. Posting of images, files, gifs and emoji will also be possible. Configuration can be on a server wide or per channel / per role basis.

#### Feeds
Announcements, Events and other sources can be displayed in feeds, whether populated by a local channel only visible to those allowed to post or through RSS, WebSocket or other means, feeds allow for nicer formating and interactions from users, like marking attendance to an event or reacting to polls.

#### Forums
A thread based channel that allows users to browse threads, reply to them and even subscribe.  These are displayed in a unique user friendly way to ensure ease of use.  Configurable sorting methods like newest, recent update or pinning.
