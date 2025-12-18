import type { StorybookConfig } from '@storybook/web-components-vite';

const config: StorybookConfig = {
  stories: ['../stories/**/*.stories.@(js|jsx|mjs|ts|tsx)'],
  addons: ['@storybook/addon-links', '@storybook/addon-docs'],
  framework: {
    name: '@storybook/web-components-vite',
    options: {},
  },
  staticDirs: [
    // Serve flag images from the parent static directory
    { from: '../../static/flags', to: '/static/flags' },
  ],
  viteFinal: async (config) => {
    return {
      ...config,
      esbuild: {
        ...config.esbuild,
        // Support for decorators
        target: 'ES2020',
      },
    };
  },
};

export default config;
