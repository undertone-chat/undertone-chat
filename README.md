# Undertone

In the world of online roleplay there are few free and flexible options for realtime communication, a critical part of the roleplay experience. Most solutions are either expensive to license or are very specific to one particular genre or game. Undertone solves these issues by providing an open source and free to use real time voice communication solution which is:
1. Easily extensible through scripting to work with any game or genre.
2. Provides out of the box solutions for the most common roleplay games and requirements.
3. Built on rust to be fast and platform agnostic.
4. Scallable server and client that can work on even bad connections and slow computers to remain inclusive of all communities.

Undertone is a community tool, built and maintained by the community and will always be free and available. Developers are welcome to fork and create their own branded or specific versions as long as they contribute changes back to the main project and remain open source and free to use as per the [LICENSE](LICENSE).  The only exception will be for companies hosting servers as a service will have to purchase licenses from the organization to help fund continued development and ensure a healthy ecosystem for the community. 

## Project Dependencies

Every great project is built on the backs of giants who came before it. Undertone is no different, though we strive to minimize the amount of dependcies to keep the project sleek and light weight, it would be irresponsibile to not re-invent every wheel we need when solutions already exist.  This list is non-exaughstive and can and will change over the lifetime of the project. For version specific dependencies be sure to check the CHANGELOG.

### Global Dependencies

- [Serde](https://crates.io/crates/serde) - for serialization of data structures across the project.

### Library's
- [thiserror](https://crates.io/crates/thiserror) - macros to assist in effective error type creations.

### Application
- [Anyhow](https://crates.io/crates/anyhow) - Easier error handling in application environments.
