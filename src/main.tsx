import React from 'react'
import ReactDOM from 'react-dom/client'
import App from '@/app'
import { Toaster } from '@/components/ui/sonner'
import { ThemeProvider } from 'next-themes'

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <ThemeProvider attribute="class" defaultTheme="system" enableSystem>
      <App />
      <Toaster />
    </ThemeProvider>
  </React.StrictMode>
)
