import {themes as prismThemes} from 'prism-react-renderer';
import type {Config} from '@docusaurus/types';
import type * as Preset from '@docusaurus/preset-classic';
import * as fs from 'fs';
import {getExamplesPath, getApiDocsUrl} from './src/utils/versions';
const siteConfig = require('./site-config.json');
const isMain = siteConfig.branch == 'main';
const editUrl = `https://github.com/substrate-labs/substrate2/tree/${siteConfig.branch}/docs/docusaurus`;

const config: Config = {
  title: 'Substrate Labs',
  tagline: '21st century electronic design automation tools, written in Rust.',
  favicon: 'img/substrate_logo_blue.png',

  // Set the production url of your site here
  url: 'https://docs.substratelabs.io',
  // Set the /<baseUrl>/ pathname under which your site is served
  // For GitHub pages deployment, it is often '/<projectName>/'
  baseUrl: isMain ? '/' : `/branch/${siteConfig.branch}/`,

  // GitHub pages deployment config.
  // If you aren't using GitHub pages, you don't need these.
  organizationName: 'substrate-labs', // Usually your GitHub org/user name.
  projectName: 'substrate', // Usually your repo name.

  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'throw',

  // Even if you don't use internalization, you can use this field to set useful
  // metadata like html lang. For example, if your site is Chinese, you may want
  // to replace "en" with "zh-Hans".
  i18n: {
    defaultLocale: 'en',
    locales: ['en'],
  },

  onBrokenLinks: 'ignore',

  plugins: ['./src/plugins/substrate-source-assets'],

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          ... !isMain && {
            onlyIncludeVersions: ['current'],
            lastVersion: 'current',
          },
          sidebarPath: require.resolve('./sidebars.js'),
          versions: {
            current: {
              label: siteConfig.branch,
              path: isMain ? siteConfig.branch : '',
              banner: 'unreleased',
            },
          },
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: editUrl,
        },
        blog: isMain ? {
          // Please change this to your repo.
          // Remove this to remove the "edit this page" links.
          editUrl: editUrl,
        } : false,
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  markdown: {
    format: "mdx",
    preprocessor: ({ filePath, fileContent }) => {
      console.log("Injecting global variables into " + filePath);
      let version;
      let match = /versioned_docs\/version-([a-zA-Z0-9_-]*)\//.exec(filePath);
      if (match) {
          version = match[1];
      } else {
          version = siteConfig.branch;
      }
      let vars = new Map([
          ["VERSION", version],
          ["EXAMPLES", getExamplesPath(version)],
          ["API", getApiDocsUrl(version)],
      ]);

      for (const [key, value] of vars) {
          fileContent = fileContent.replaceAll(`{{${key}}}`, value);
      }
      return fileContent;
    },
  },

  themeConfig: {
      // Replace with your project's social card
      image: 'img/substrate_logo.png',
      navbar: {
        title: 'Substrate Labs',
        logo: {
          alt: 'Substrate Labs Logo',
          src: 'img/substrate_logo.png',
          srcDark: 'img/substrate_logo_dark.png',
        },
        items: isMain ? [
          {
            type: 'docSidebar',
            sidebarId: 'tutorialSidebar',
            position: 'left',
            label: 'Documentation',
          },
          {
            type: 'custom-apiLink',
            position: 'left',
          },
          {to: 'blog', label: 'Blog', position: 'left'},
          {
            type: 'docsVersionDropdown',
            position: 'right',
          },
          {
            href: 'https://github.com/substrate-labs/substrate2',
            label: 'GitHub',
            position: 'right',
          },
        ] : [
          {
            type: 'docSidebar',
            sidebarId: 'tutorialSidebar',
            position: 'left',
            label: 'Documentation',
          },
          {
            type: 'custom-apiLink',
            position: 'left',
          },
          {
            type: 'docsVersion',
            position: 'right',
          },
          {
            href: `https://github.com/substrate-labs/substrate2/tree/${siteConfig.branch}`,
            label: 'GitHub',
            position: 'right',
          },
        ],
      },
      footer: {
        style: 'dark',
        links: [],
        copyright: `Copyright © ${new Date().getFullYear()} Substrate Labs. Built with Docusaurus.`,
      },
    prism: {
      theme: prismThemes.oneLight,
      darkTheme: prismThemes.palenight, // nightowl
      additionalLanguages: ['rust', 'toml'],
    },
  } satisfies Preset.ThemeConfig,
};

export default config;
