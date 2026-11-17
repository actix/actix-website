import { Config } from "@docusaurus/types";
import type * as Preset from "@docusaurus/preset-classic";

import { themes as prismThemes } from "prism-react-renderer";
import englishMessages from "./src/data/site-messages.json";
import spanishMessages from "./i18n/es/site.json";

const draculaTheme = prismThemes.dracula;
const locale = process.env.DOCUSAURUS_CURRENT_LOCALE ?? "en";
const messages = locale === "es" ? spanishMessages : englishMessages;

const config: Config = {
  title: "Actix Web",
  tagline: messages.tagline,
  url: "https://actix.rs",
  baseUrl: "/",
  trailingSlash: true,
  i18n: {
    defaultLocale: "en",
    locales: ["en", "es"],
    localeConfigs: {
      en: { label: "English" },
      es: { label: "Español" },
    },
  },
  onBrokenLinks: "throw",
  favicon: "img/logo.png",
  organizationName: "actix", // Usually your GitHub org/user name.
  projectName: "actix-web", // Usually your repo name.
  // https://docusaurus.io/docs/api/themes/@docusaurus/theme-mermaid
  // https://docusaurus.io/docs/markdown-features/diagrams
  markdown: {
    mermaid: true,
    hooks: {
      onBrokenMarkdownLinks: "warn",
    },
  },
  themes: [
    "@docusaurus/theme-mermaid",
    [
      require.resolve("@easyops-cn/docusaurus-search-local"),
      {
        hashed: true,
        language: locale === "es" ? ["es", "en"] : "en",
        docsRouteBasePath: ["docs", "community"],
        docsDir: ["docs", "community"],
        indexBlog: false,
        indexPages: true,
        ignoreFiles: /^docs\/actix(?:\/|$)/,
      },
    ],
  ],
  themeConfig: {
    image: "img/actix-web-social.png",
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
          type: "localeDropdown",
          position: "right",
        },
        {
          href: "https://discord.gg/NWpN5mmg3x",
          position: "right",
          className: "header-discord-link",
          "aria-label": messages.discord,
        },
        {
          href: "https://crates.io/crates/actix-web",
          position: "right",
          className: "header-crates-link",
          "aria-label": messages.crates,
          title: messages.crates,
        },
        {
          href: "https://docs.rs/actix-web",
          position: "right",
          className: "header-docs-link",
          "aria-label": messages.api,
          title: messages.api,
        },
        {
          href: "https://github.com/actix/actix-web",
          position: "right",
          className: "header-github-link",
          "aria-label": messages.github,
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
    [
      "@docusaurus/plugin-content-docs",
      {
        id: "community",
        path: "community",
        routeBasePath: "community",
        sidebarPath: require.resolve("./communitySidebars.ts"),
        editUrl: "https://github.com/actix/actix-website/edit/main/",
        editLocalizedFiles: true,
      },
    ],
  ],
  presets: [
    [
      "classic",
      {
        blog: false,
        docs: {
          sidebarPath: require.resolve("./sidebars.js"),
          sidebarCollapsed: false,
          editUrl: "https://github.com/actix/actix-website/edit/main/",
          editLocalizedFiles: true,
        },
        theme: {
          customCss: require.resolve("./src/css/custom.css"),
        },
      } satisfies Preset.Options,
    ],
  ],
};

export default config;
