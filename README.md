<div align="center"><img src="docs/images/icon_1024x1024.png" width="30%"></div>

***TABLE OF CONTENTS***

<!-- TOC -->

- [1. Undertone](#1-undertone)
- [2. Key Features](#2-key-features)
  - [2.1. Voice Communication](#21-voice-communication)
  - [2.2. Text Communication](#22-text-communication)
  - [2.3. Control and Privacy](#23-control-and-privacy)
  - [2.4. Realtime Game Data and Extensibility](#24-realtime-game-data-and-extensibility)
- [3. Financing and Licensing](#3-financing-and-licensing)
- [4. Contributing](#4-contributing)
- [5. Appendicies](#5-appendicies)

<!-- /TOC -->
# Undertone
Voice comms and community tools for serious gaming and roleplay. Undertone is designed to fill a niche in the gaming world where communities find them selves spread across many different solutions for voice, chat, scheduling and specialized tools used by roleplay and sim communities.3

# Key Features
These are some of  the key features that are planned for implementation, the list is not exaughstive and may not reflect the most current development plans.

## Voice Communication

The core of the software is supporting voice comms for everything from casual chat to specialized solutions for milsim and roleplay in games in FiveM, RedM, Flight Sims, Arma and more.

- [ ] Channel based voice chat for community and casual gaming  like you might find on Discord or TeamSpeak.
- [ ] World mode: positional 3D
- [ ] SubMix mode: Customizable voice channels used to simulate radios, cell phones and even supernatural telepathy, you control the effects and rules of how it is rendered.

## Text Communication

No community solution is complete without the ability to chat, make posts, and create announcements to keep your players in sync and engaged. All text chat is planned to support almost all of the MarkDown options and extensions like LaTeX and Mermaid.

- [ ] Text channels for persistent scrolling chat and live conversations.
- [ ] Forum channels for posting threads and organizing communications more effectively with stronger persistence for things like support and recruiting.
- [ ] Feed channels where you can use sources like a Text channel, rss feed or websocket and format them based on your own creative choices for display to the community.
- [ ] Events with server calendars, signups and reminders.

## Control and Privacy

One of the primary goals of Undertone was to provide communities with a tool that handles most of their needs and prevent facturing across multiple apps and platforms. A big advantage of that is controling where and how your information is stored as well having full control over your hosting and integrations.

- [ ] End to End encrytpion for all network traffic.
- [ ] Private data always stored with encryption in the database so you dont have to worry.
- [ ] Fully encrypted private chats with rotating keys so only those present when a message was written can decrypt it. No worries about snooping admins or data breaches for your personal chats.
- [ ] You choose your hosting platform whether that is at home or on the cloud, or next to your roleplay server for tight integration.

## Realtime Game Data and Extensibility

In order to serve different games, different communities and all the needs that fall in between the extremes, Undertone supports everal path ways for integration and capturing the data you need.

- [ ] Api for receiving data from the client via mods.
- [ ] Lua scripting to poll data from game API or server API.
- [ ] Highly configurable systems for parsing and tweaking data on the server to drive effects like radio distortion or reverb.

# Financing and Licensing

It was critical that Undertone's source code be available to the community so they could see exactly how secure and safe the software was for clients and servers, as well that there were protections to prevent people from trying to profit unfairly from the product.  To solve this we are using a Source Available license known as a Business Source License. This achieves a couple important things.

1. Ensures the source code is open and available for modifications allowing the community to help maintain and enhance.
2. Allows for individual communities to run their own modified version based on the main source, but not distribute it to compete or make money.
3. Strict control over when and how profits can be made with the software. Unlike `OpenSource` licenses which have no restrictions on when or how a product or code can be used, this license lets us protect the product and ensure that if someone wants to make money hosting servers the project will benefit financially through licensing agreements. These kinds of things will go a long way towards the longevity of the development.
4. You as a self hosted / end user will never have to pay for access or features. We will not commit to saying there will never be any kind of in app purchases say for cosmetics, but we will never hide featurs behind pay walls like boosting or subscriptions.

# Contributing

At this time we are still evaluating how we plan to take on contributors, but you will always be able to fork the repository and submit pull requests, or submit issues if you find a problem or have a great idea for a new feature.

# Appendicies
1. [Style Guide](docs/styleguide.md)
2. [Protocol Design](docs/protocol.md)