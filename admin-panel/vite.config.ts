import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  server: {
    port: 5173,
    proxy: {
      '/api': 'http://localhost:30121',
      '/metrics': 'http://localhost:30121'
    }
  },
  build: {
    outDir: 'dist'
  }
}); 