import { StrictMode, BrowserRouter } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './task_itog.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <BrowserRouter>
      <App />
    </BrowserRouter>
  </StrictMode>,
)
