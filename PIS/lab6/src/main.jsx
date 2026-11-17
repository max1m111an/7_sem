import { StrictMode, Suspense, lazy } from 'react'
import { createRoot } from 'react-dom/client'
import { BrowserRouter } from 'react-router-dom'

// Only the app selected by --mode is loaded, so the other app's CSS is
// never injected into the page. This lets `task` and `itog` run side by
// side without their global styles (body, header, h1, ...) overlaying.
const App = lazy(() =>
  import.meta.env.MODE === 'itog'
    ? import('./task_itog.jsx').then((m) => ({ default: m.App }))
    : import('./task.jsx').then((m) => ({ default: m.App })),
)

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <BrowserRouter>
      <Suspense fallback={<div style={{ padding: 24 }}>Загрузка…</div>}>
        <App />
      </Suspense>
    </BrowserRouter>
  </StrictMode>,
)
