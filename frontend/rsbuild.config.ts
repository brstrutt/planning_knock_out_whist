import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';

export default defineConfig({
  plugins: [pluginReact()],
  output: {
    distPath: {
      root: '../backend/public', // Adjust path as needed
    },
    cleanDistPath: true,
  },
  html: {
    title: 'Planning KNOCK OUT WHIST',
    favicon: './src/assets/favicon.ico',
  },
});
