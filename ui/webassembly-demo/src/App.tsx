import { useState, useEffect } from 'react'
import {
  AppBar,
  Toolbar,
  Typography,
  Container,
  Box,
  Tabs,
  Tab,
  Paper,
  Chip
} from '@mui/material'
import { motion, AnimatePresence } from 'framer-motion'
import SpeedIcon from '@mui/icons-material/Speed'
import MemoryIcon from '@mui/icons-material/Memory'
import CompareIcon from '@mui/icons-material/Compare'
import AccountBalanceIcon from '@mui/icons-material/AccountBalance'

// Components
import BankingDemo from './components/BankingDemo'
import PerformanceMonitor from './components/PerformanceMonitor'
import MemoryComparison from './components/MemoryComparison'
import WebAssemblyBench from './components/WebAssemblyBench'

// Types
interface TabPanelProps {
  children?: React.ReactNode
  index: number
  value: number
}

function TabPanel(props: TabPanelProps) {
  const { children, value, index, ...other } = props

  return (
    <div
      role="tabpanel"
      hidden={value !== index}
      id={`demo-tabpanel-${index}`}
      aria-labelledby={`demo-tab-${index}`}
      {...other}
    >
      {value === index && (
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          exit={{ opacity: 0, y: -20 }}
          transition={{ duration: 0.3 }}
        >
          {children}
        </motion.div>
      )}
    </div>
  )
}

function App() {
  const [currentTab, setCurrentTab] = useState(0)
  const [isLoaded, setIsLoaded] = useState(false)
  const [wasmSupport, setWasmSupport] = useState(false)

  useEffect(() => {
    // Check WebAssembly support
    const checkWasmSupport = () => {
      try {
        if (typeof WebAssembly === 'object' && typeof WebAssembly.instantiate === 'function') {
          setWasmSupport(true)
          console.log('✅ WebAssembly is supported')
        } else {
          console.warn('⚠️ WebAssembly is not supported')
        }
      } catch (error) {
        console.error('❌ Error checking WebAssembly support:', error)
      }
    }

    checkWasmSupport()
    
    // Simulate app loading
    const timer = setTimeout(() => {
      setIsLoaded(true)
    }, 500)

    return () => clearTimeout(timer)
  }, [])

  const handleTabChange = (_event: React.SyntheticEvent, newValue: number) => {
    setCurrentTab(newValue)
  }

  const tabs = [
    {
      label: 'Banking Demo',
      icon: <AccountBalanceIcon />,
      component: <BankingDemo />
    },
    {
      label: 'Performance',
      icon: <SpeedIcon />,
      component: <PerformanceMonitor />
    },
    {
      label: 'Memory Usage',
      icon: <MemoryIcon />,
      component: <MemoryComparison />
    },
    {
      label: 'WASM Benchmark',
      icon: <CompareIcon />,
      component: <WebAssemblyBench />
    }
  ]

  if (!isLoaded) {
    return (
      <Box 
        display="flex" 
        justifyContent="center" 
        alignItems="center" 
        minHeight="100vh"
        bgcolor="background.default"
      >
        <motion.div
          initial={{ scale: 0.5, opacity: 0 }}
          animate={{ scale: 1, opacity: 1 }}
          transition={{ duration: 0.5 }}
        >
          <Typography variant="h4" color="primary">
            Загрузка GameVerse UI...
          </Typography>
        </motion.div>
      </Box>
    )
  }

  return (
    <Box sx={{ flexGrow: 1, bgcolor: 'background.default', minHeight: '100vh' }}>
      {/* Performance indicator */}
      <PerformanceIndicator />
      
      {/* App Bar */}
      <AppBar position="static" elevation={0} sx={{ bgcolor: 'background.paper' }}>
        <Toolbar>
          <motion.div
            initial={{ x: -50, opacity: 0 }}
            animate={{ x: 0, opacity: 1 }}
            transition={{ duration: 0.5 }}
            style={{ display: 'flex', alignItems: 'center', flexGrow: 1 }}
          >
            <Typography variant="h6" component="div" sx={{ mr: 2, fontWeight: 700 }}>
              🎮 GameVerse Framework
            </Typography>
            <Typography variant="subtitle1" color="text.secondary">
              WebAssembly UI Demo
            </Typography>
          </motion.div>
          
          <Box sx={{ display: 'flex', gap: 1 }}>
            <Chip
              label={wasmSupport ? "WASM ✅" : "WASM ❌"}
              color={wasmSupport ? "primary" : "error"}
              size="small"
            />
            <Chip
              label="vs FiveM CEF"
              color="secondary"
              size="small"
            />
          </Box>
        </Toolbar>
      </AppBar>

      {/* Navigation Tabs */}
      <Box sx={{ borderBottom: 1, borderColor: 'divider', bgcolor: 'background.paper' }}>
        <Container maxWidth="lg">
          <Tabs
            value={currentTab}
            onChange={handleTabChange}
            aria-label="demo navigation tabs"
            variant="fullWidth"
            textColor="primary"
            indicatorColor="primary"
          >
            {tabs.map((tab, index) => (
              <Tab
                key={index}
                icon={tab.icon}
                label={tab.label}
                id={`demo-tab-${index}`}
                aria-controls={`demo-tabpanel-${index}`}
                sx={{
                  textTransform: 'none',
                  fontWeight: 600,
                  '&.Mui-selected': {
                    color: 'primary.main'
                  }
                }}
              />
            ))}
          </Tabs>
        </Container>
      </Box>

      {/* Tab Content */}
      <Container maxWidth="lg" sx={{ py: 3 }}>
        <AnimatePresence mode="wait">
          {tabs.map((tab, index) => (
            <TabPanel key={index} value={currentTab} index={index}>
              <Paper 
                elevation={2} 
                sx={{ 
                  p: 3, 
                  bgcolor: 'background.paper',
                  borderRadius: 2,
                  border: '1px solid',
                  borderColor: 'divider'
                }}
              >
                {tab.component}
              </Paper>
            </TabPanel>
          ))}
        </AnimatePresence>
      </Container>
    </Box>
  )
}

// Performance indicator component
function PerformanceIndicator() {
  const [performance, setPerformance] = useState({
    fps: 60,
    memory: 0,
    loadTime: 0
  })

  useEffect(() => {
    const updatePerformance = () => {
      const perf = (window as any).gameVersePerformance || {}
      const memory = (performance as any).memory?.usedJSHeapSize || 0
      
      setPerformance({
        fps: Math.floor(60 + Math.random() * 10), // Simulated FPS
        memory: Math.round(memory / 1024 / 1024 * 100) / 100,
        loadTime: perf.loadTime || 0
      })
    }

    updatePerformance()
    const interval = setInterval(updatePerformance, 1000)

    return () => clearInterval(interval)
  }, [])

  return (
    <div className="performance-indicator">
      <div>FPS: {performance.fps}</div>
      <div>RAM: {performance.memory}MB</div>
      <div>Load: {performance.loadTime.toFixed(0)}ms</div>
    </div>
  )
}

export default App 