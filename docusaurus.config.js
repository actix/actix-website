const path = require('path');

module.exports = {
  title: 'Actix',
  tagline: 'A powerful, pragmatic, and extremely fast web framework for Rust',
  url: 'https://actix.rs',
  baseUrl: '/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',
  favicon: 'img/logo.png',
  organizationName: 'actix', // Usually your GitHub org/user name.
  projectName: 'actix-web', // Usually your repo name.
  themeConfig: {
    navbar: {
      title: 'Actix',
      logo: {
        alt: 'Actix Logo',
        src: 'img/logo.png',
      },
      items: [
        {
          to: 'docs',
          activeBasePath: 'docs',
          label: 'Documentation',
          position: 'left',
        },
        {
          to: 'community',
          activeBasePath: 'community',
          label: 'Community',
          position: 'left',
        },
        {
          to: 'code',
          activeBasePath: 'code',
          label: 'Code',
          position: 'left',
        },
      ],
    },
    footer: {
      style: 'dark',
      copyright: `Copyright © ${new Date().getFullYear()} The Actix Team`,
    },
    prism: {
        additionalLanguages: ['rust', 'toml'],
        defaultLanguage: 'rust'
    }
  },
  presets: [
    [
      '@docusaurus/preset-classic',
      {
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl:
            'https://github.com/actix/actix-website/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      },
    ],
  ],
};
