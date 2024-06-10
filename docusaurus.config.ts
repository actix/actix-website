import { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

import { themes as prismThemes } from "prism-react-renderer";

const draculaTheme = prismThemes.dracula;

const config: Config = {
  title: "Actix Web",
  tagline:
    "Actix Web is a powerful, pragmatic, and extremely fast web framework for Rust",
  url: "https://actix.rs",
  baseUrl: "/",
  onBrokenLinks: "throw",
  onBrokenMarkdownLinks: "warn",
  favicon: "img/logo.png",
  organizationName: "actix", // Usually your GitHub org/user name.
  projectName: "actix-web", // Usually your repo name.
  // https://docusaurus.io/docs/api/themes/@docusaurus/theme-mermaid
  // https://docusaurus.io/docs/markdown-features/diagrams
  markdown: {
    mermaid: true,
  },
  themes: ["@docusaurus/theme-mermaid"],
  themeConfig: {
    navbar: {
      title: "Actix Web",
      logo: {
        src: "img/logo-icon.png",
        width: 32,
        height: 32,
        alt: "Actix Web Logo",
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
        {
          href: "https://discord.gg/NWpN5mmg3x",
          position: "right",
          className: "header-discord-link",
          "aria-label": "Chat on Discord",
        },
        {
          href: "https://github.com/actix/actix-web",
          position: "right",
          className: "header-github-link",
          "aria-label": "GitHub repository",
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
  } satisfies Preset.ThemeConfig,
  plugins: [
    "docusaurus-plugin-sass",
    require.resolve("docusaurus-lunr-search"),
  ],
  presets: [
    [
      "classic",
      {
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          editUrl: "https://github.com/actix/actix-website/edit/main/",
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      } satisfies Preset.Options,
    ],
  ],
};

export default config;
