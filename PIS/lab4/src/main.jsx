import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import './index.css'
import App from './task_itog.jsx'

createRoot(document.getElementById('root')).render(
  <StrictMode>
    <App />
  </StrictMode>,
)
