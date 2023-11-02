# Actix Website

## Getting Started

Building the website depends on [Docusaurus][docusaurus], you must have `npm` or `yarn` installed. You can run the site locally with:

```sh
git clone https://github.com/actix/actix-website.git
cd actix-website
npm install  # or yarn install
npm start  # or yarn start
```

Then visit http://localhost:3000.

## Updating diagrams

Diagrams are located under [/static/img/diagrams/](https://github.com/actix/actix-website/tree/master/static/img/diagrams) and built with [Mermaid CLI][mermaid_cli].

For instance to edit `connection_overview` diagram:

```sh
cd static/img/diagrams
vi connection_overview.mmd
# Compile diagrams:
mmdc -i connection_overview.mmd -o connection_overview.svg
```

## License

This site is licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0])
- MIT license ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT])

<!-- LINKS -->

[docusaurus]: https://docusaurus.io/
[mermaid_cli]: https://github.com/mermaidjs/mermaid.cli

