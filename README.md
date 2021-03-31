# Actix Website

## Getting Started

Building the website depends on [Docusaurus], so you must have yarn/npm installed. You can run the site locally with:

```sh
git clone https://github.com/actix/actix-website.git
cd actix-website

yarn start OR npm run start
```

Then visit http://localhost:1313.

## Updating diagrams

Diagrams are located under [/static/img/diagrams/](https://github.com/actix/actix-website/tree/master/static/img/diagrams) and built with [Mermaid CLI].

For instance to edit `connection_overview` diagram:

```sh
cd static/img/diagrams
vi connection_overview.mmd
# Compile diagrams:
mmdc -i connection_overview.mmd -o connection_overview.svg
```

# License

This site is licensed under either of

* Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or
  [http://www.apache.org/licenses/LICENSE-2.0])
* MIT license ([LICENSE-MIT](LICENSE-MIT) or
  [http://opensource.org/licenses/MIT])

<!-- LINKS -->

[Hugo]: https://docusaurus.io/
