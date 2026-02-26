import {themes as prismThemes} from 'prism-react-renderer';

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'forge',
  tagline: 'competitive programming, without the boilerplate.',
  favicon: 'img/favicon.ico',

  future: {
    v4: true,
  },

  url: 'https://forge.afpereira.me',
  baseUrl: '/',

  onBrokenLinks: 'throw',

  i18n: {
    defaultLocale: 'en',
    locales: ['en', 'pt'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: './sidebars.js',
          editUrl: 'https://github.com/afonp/forge/tree/main/website/',
        },
        blog: false,
        theme: {
          customCss: './src/css/custom.css',
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      colorMode: {
        defaultMode: 'dark',
        respectPrefersColorScheme: true,
      },
      navbar: {
        title: 'forge',
        items: [
          {
            type: 'docSidebar',
            sidebarId: 'tutorialSidebar',
            position: 'left',
            label: 'docs',
          },
          {
            href: 'https://github.com/afonp/forge',
            label: 'github',
            position: 'right',
          },
          {
            type: 'localeDropdown',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [
          {
            title: 'docs',
            items: [
              {
                label: 'getting started',
                to: '/docs/intro',
              },
            ],
          },
          {
            title: 'links',
            items: [
              {
                label: 'github',
                href: 'https://github.com/afonp/forge',
              },
            ],
          },
        ],
        copyright: `built by afonso`,
      },
      prism: {
        theme: prismThemes.github,
        darkTheme: prismThemes.dracula,
        additionalLanguages: ['cpp', 'bash', 'makefile', 'toml', 'rust'],
      },
    }),
};

export default config;
