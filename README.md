# Identicons

*Identicons* are images that are generated from a seed value in a
consistent but unpredictable way. This is usually accomplished by
seeding a random number generator, and then getting values from it in
a consistent way.

Identicons are useful for automatically assigning images to items in a
CRUD interface, giving new users a thematic avatar, or any other place
where having distinguishable icons is useful, but choosing them by
hand would be too tedious.

This project is a Rust web service that generates identicons. Here are
some examples:

* Identicon: <img src="https://identicons.appspot.com/i/shield/v1/Identicon.svg">
* Alpha: <img src="https://identicons.appspot.com/i/shield/v1/Alpha.svg">
* Beta: <img src="https://identicons.appspot.com/i/shield/v1/Beta.svg">
* Gamma: <img src="https://identicons.appspot.com/i/shield/v1/Gamma.svg">
