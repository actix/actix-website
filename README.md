# Actix Website

## Getting Started

Building the website depends on [Docusaurus][docusaurus], you must have `npm` or `yarn` installed. You can run the site locally with:

```sh
git clone https://github.com/actix/actix-website.git
cd actix-website
npm install  # or yarn install
npm start  # or yarn start
```

Then visit <http://localhost:3000>.

## Updating Project Activity

The homepage imports `src/data/project-activity.json` at build time. It does not request GitHub data in the browser. Dates use UTC and remain fixed until the next deployment.

The **Update Project Activity** workflow runs daily and can also run manually. It opens or updates `chore/project-activity` with the latest star count, stable Actix Web release, and default-branch commit. Merge the pull request to deploy through Netlify.

Enable **Allow GitHub Actions to create and approve pull requests** in the repository's Actions settings. Pull requests created with `GITHUB_TOKEN` do not automatically trigger other GitHub Actions workflows. Close and reopen the pull request as a maintainer to trigger the normal checks before merge. See [GitHub's workflow trigger documentation](https://docs.github.com/en/actions/how-tos/write-workflows/choose-when-workflows-run/trigger-a-workflow).

To update the snapshot locally:

```sh
node scripts/update-project-activity.mjs
```

Set `GITHUB_TOKEN` to use authenticated API requests. Failed requests or invalid responses leave the previous snapshot intact. Normal site builds use the saved snapshot and do not need GitHub access.

## Updating Diagrams

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

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

<!-- LINKS -->

[docusaurus]: https://docusaurus.io/
[mermaid_cli]: https://github.com/mermaidjs/mermaid.cli
