const path = require("path");
const {
  themes: { dracula: draculaTheme },
} = require("prism-react-renderer");

module.exports = {
  title: "Actix",
  tagline:
    "Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust",
  url: "https://actix.rs",
  baseUrl: "/",
  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",
  favicon: "img/logo.png",
  organizationName: "actix", // Usually your GitHub org/user name.
  projectName: "actix-web", // Usually your repo name.
  themeConfig: {
    navbar: {
      title: "Actix",
      logo: {
        alt: "Actix Logo",
        src: "img/logo-icon.png",
        width: 32,
        height: 32,
      },
      items: [
        {
          to: "docs",
          activeBasePath: "docs",
          label: "Documentation",
          position: "left",
        },
        {
          to: "community",
          activeBasePath: "community",
          label: "Community",
          position: "left",
        },
        {
          to: "code",
          activeBasePath: "code",
          label: "Code",
          position: "left",
        },
      ],
    },
    footer: {
      copyright: `Copyright © ${new Date().getFullYear()} The Actix Team`,
    },
    prism: {
      // dracula is closest to docs.rs, where keywords are highlighted
      theme: draculaTheme,
      additionalLanguages: ["rust", "toml", "shell-session"],
      defaultLanguage: "rust",
    },
    colorMode: {
      respectPrefersColorScheme: true,
    },
  },
  plugins: [
    "docusaurus-plugin-sass",
    require.resolve("docusaurus-lunr-search"),
  ],
  presets: [
    [
      "@docusaurus/preset-classic",
      {
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          editUrl: "https://github.com/actix/actix-website/edit/main/",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      },
    ],
  ],
};
