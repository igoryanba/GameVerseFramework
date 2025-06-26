import { StrictMode } from 'react'
import { createRoot } from 'react-dom/client'
import { ThemeProvider, createTheme } from '@mui/material/styles'
import CssBaseline from '@mui/material/CssBaseline'
import App from './App.tsx'
import './styles/global.css'

// GameVerse Material-UI theme
const gameVerseTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: '#00ff88',
      dark: '#00cc6a',
      light: '#33ff9a'
    },
    secondary: {
      main: '#ff6b00',
      dark: '#cc5500',
      light: '#ff8533'
    },
    background: {
      default: '#1a1a2e',
      paper: '#16213e'
    },
    text: {
      primary: '#ffffff',
      secondary: '#cccccc'
    }
  },
  typography: {
    fontFamily: 'Roboto, Arial, sans-serif',
    h1: {
      fontWeight: 700,
      fontSize: '2.5rem'
    },
    h2: {
      fontWeight: 600,
      fontSize: '2rem'
    }
  },
  components: {
    MuiButton: {
      styleOverrides: {
        root: {
          textTransform: 'none',
          borderRadius: 8,
          fontWeight: 600
        }
      }
    },
    MuiCard: {
      styleOverrides: {
        root: {
          backgroundImage: 'linear-gradient(135deg, rgba(255,255,255,0.1) 0%, rgba(255,255,255,0.05) 100%)',
          backdropFilter: 'blur(10px)',
          border: '1px solid rgba(255,255,255,0.1)'
        }
      }
    }
  }
})

// Performance monitoring
const startTime = performance.now()

async function initializeApp() {
  try {
    // Initialize WebAssembly modules if available
    console.log('🚀 Initializing GameVerse WebAssembly UI...')
    
    // Performance tracking
    const memoryBefore = (performance as any).memory?.usedJSHeapSize || 0
    
    const root = createRoot(document.getElementById('root')!)
    
    root.render(
      <StrictMode>
        <ThemeProvider theme={gameVerseTheme}>
          <CssBaseline />
          <App />
        </ThemeProvider>
      </StrictMode>
    )
    
    // Hide loading screen
    const loadingElement = document.getElementById('loading')
    if (loadingElement) {
      loadingElement.style.opacity = '0'
      setTimeout(() => {
        loadingElement.remove()
      }, 300)
    }
    
    // Performance metrics
    const loadTime = performance.now() - startTime
    const memoryAfter = (performance as any).memory?.usedJSHeapSize || 0
    const memoryUsed = (memoryAfter - memoryBefore) / 1024 / 1024 // MB
    
    console.log(`✅ GameVerse UI loaded in ${loadTime.toFixed(2)}ms`)
    console.log(`📊 Memory usage: ${memoryUsed.toFixed(2)}MB`)
    
    // Send performance data to global scope for comparison
    ;(window as any).gameVersePerformance = {
      loadTime,
      memoryUsed: memoryAfter,
      startTime,
      wasmLoaded: false // Will be updated when WASM modules load
    }
    
  } catch (error) {
    console.error('❌ Failed to initialize GameVerse UI:', error)
    
    // Fallback error display
    document.getElementById('root')!.innerHTML = `
      <div style="padding: 20px; text-align: center; color: #ff6b6b;">
        <h2>🚫 Initialization Error</h2>
        <p>Failed to load GameVerse WebAssembly UI</p>
        <pre style="background: rgba(0,0,0,0.3); padding: 10px; border-radius: 4px;">
          ${error}
        </pre>
      </div>
    `
  }
}

// Initialize the application
initializeApp() 