import type { Preview } from '@storybook/web-components-vite';
import { initialize, mswLoader } from 'msw-storybook-addon';

// Initialize MSW for API mocking
initialize();

const preview: Preview = {
  parameters: {
    controls: {
      matchers: {
        color: /(background|color)$/i,
        date: /Date$/i,
      },
    },
    // Default background options
    backgrounds: {
      options: {
        light: { name: 'light', value: '#ffffff' },
        gray: { name: 'gray', value: '#f3f4f6' },
        dark: { name: 'dark', value: '#1f2937' }
      }
    },
    // Layout configuration
    layout: 'centered',
  },

  // MSW loader for API mocking
  loaders: [mswLoader],

  // Global decorators
  decorators: [
    (story) => {
      // Add custom CSS properties used by components
      const styles = document.createElement('style');
      styles.textContent = `
        :root {
          --gray-50: #f9fafb;
          --gray-100: #f3f4f6;
          --gray-200: #e5e7eb;
          --gray-300: #d1d5db;
          --gray-400: #9ca3af;
          --gray-500: #6b7280;
          --gray-600: #4b5563;
          --gray-700: #374151;
          --gray-800: #1f2937;
          --gray-900: #111827;
          --primary: #3b82f6;
          --primary-50: #eff6ff;
          --primary-600: #2563eb;
          --success: #10b981;
          --warning: #f59e0b;
          --danger: #ef4444;
          --info: #06b6d4;
        }
        
        body {
          font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
        }
      `;
      if (!document.head.querySelector('#storybook-global-styles')) {
        styles.id = 'storybook-global-styles';
        document.head.appendChild(styles);
      }
      return story();
    },
  ],

  initialGlobals: {
    backgrounds: {
      value: 'light'
    }
  }
};

export default preview;
