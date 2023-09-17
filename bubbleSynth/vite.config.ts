import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'

// https://vitejs.dev/config/
export default defineConfig({
  server: {
    strictPort: true,
  },
  clearScreen: false,
  plugins: [react()],

  envPrefix: ["VITE_", "TAURI_"],
  build:{
    target: ["es2021", "chrome100", "safari13"],
  }
})
