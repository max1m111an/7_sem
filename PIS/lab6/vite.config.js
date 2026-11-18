import react from '@vitejs/plugin-react'
import { defineConfig, loadEnv } from 'vite'

// https://vite.dev/config/
// --mode task  → http://localhost:5173  (lab 5 app: src/task.jsx)
// --mode itog  → http://localhost:5174  (final app: src/task_itog.jsx)
// Both dev servers proxy /api → Express (task.js), so the browser never
// needs a hardcoded "http://localhost:3000" URL.
export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '')
  const isItog = mode === 'itog'
  const apiTarget = env.VITE_API_TARGET || `http://localhost:${env.PORT || 3000}`

  const proxy = {
    '/api': {
      target: apiTarget,
      changeOrigin: true,
    },
  }

  return {
    plugins: [react()],
    server: {
      port: 5173,
      strictPort: true,
      proxy,
    },
    preview: {
      port: 4173,
      strictPort: true,
      proxy,
    },
  }
})
