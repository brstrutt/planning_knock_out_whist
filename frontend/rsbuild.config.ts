import { defineConfig } from '@rsbuild/core';
import { pluginReact } from '@rsbuild/plugin-react';

export default defineConfig({
  plugins: [pluginReact()],
  output: {
    distPath: {
      root: '../backend/public',  // Adjust path as needed
    },
    cleanDistPath: true,
  },
  server: {
    proxy: {
      '/api': {
        target: 'http://backend:8080',
        changeOrigin: true,
				secure: false,
      }
    },
  },
});
