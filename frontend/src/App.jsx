/**
 * Main Application component for Jellynizer.
 * Coordinates state management and layout.
 */
import React, { useState, useEffect, useRef } from 'react'
import axios from 'axios'
import Header from './components/Header'
import LibraryExplorer from './components/LibraryExplorer'
import PreviewSection from './components/PreviewSection'
import ConfirmationDialog from './components/ConfirmationDialog'
import SuccessDialog from './components/SuccessDialog'
import ProgressDialog from './components/ProgressDialog'

const API_BASE = import.meta.env.VITE_API_BASE_URL || '/api'

function App() {
   const [media, setMedia] = useState([])
   const [isScanning, setIsScanning] = useState(false)
   const [isProcessing, setIsProcessing] = useState(false)
   const [isOrganizing, setIsOrganizing] = useState(false)
    const [previewMoves, setPreviewMoves] = useState([])
    const [selectedPaths, setSelectedPaths] = useState(new Set())
    const [organizeProgress, setOrganizeProgress] = useState({
     status: 'Idle',
     current: 0,
     total: 0,
     current_file: null
   })
   const organizeProgressRef = useRef(organizeProgress)

   useEffect(() => {
     organizeProgressRef.current = organizeProgress
   }, [organizeProgress])
  const [error, setError] = useState(null)
  const [filter, setFilter] = useState('all') // 'all' | 'movies' | 'shows'
  const [isSuccessDialogOpen, setIsSuccessDialogOpen] = useState(false)
   const [successMessage, setSuccessMessage] = useState('')
 
   const fetchMedia = async () => {
    try {
      const response = await axios.get(`${API_BASE}/media`)
      setMedia(response.data)
    } catch (err) {
      console.error('Failed to fetch media', err)
    }
  }

   const fetchStatus = async () => {
     try {
       const response = await axios.get(`${API_BASE}/status`)
       setIsScanning(response.data.is_scanning)
     } catch (err) {
       console.error('Failed to fetch status', err)
     }
   }

   const fetchOrganizeStatus = async () => {
     try {
       const response = await axios.get(`${API_BASE}/organize-status`)
       const newProgress = response.data

       if (newProgress.status === 'Running') {
         setIsOrganizing(true)
         setIsProcessing(true)
         // If we are already in an active organization session (e.g. via SSE),
         // don't overwrite the detailed progress to prevent the "0% bug" 
         // caused by polling potentially older state.
         if (!isOrganizing) {
           setOrganizeProgress(newProgress)
         } else {
           setOrganizeProgress(prev => ({
             ...prev,
             status: newProgress.status
           }))
         }
       } else {
         setOrganizeProgress(newProgress)
         setIsOrganizing(false)
         if (newProgress.status !== 'Running') {
           setIsProcessing(false)
         }
       }
     } catch (err) {
       console.error('Failed to fetch organize status', err)
     }
   }
 
   const triggerScan = async () => {
    console.log('triggerScan called')
    setIsScanning(true)
    try {
      setError(null)
      setPreviewMoves([])
      const response = await axios.post(`${API_BASE}/scan`)
      if (response.data.status === 'success') {
        const pollStatus = setInterval(async () => {
          try {
            const statusRes = await axios.get(`${API_BASE}/status`)
            setIsScanning(statusRes.data.is_scanning)
            if (!statusRes.data.is_scanning) {
              clearInterval(pollStatus)
              fetchMedia()
            }
          } catch (e) {
            clearInterval(pollStatus)
          }
        }, 2000)
      } else {
        setError(response.data.message)
      }
    } catch (err) {
      setError('Failed to initiate scan')
      setIsScanning(false)
    }
  }

  const fetchPreview = async () => {
    try {
      setError(null)
      setIsProcessing(true)
      const response = await axios.get(`${API_BASE}/preview`)
      setPreviewMoves(response.data)
    } catch (err) {
      setError('Failed to fetch preview')
    } finally {
      setIsProcessing(false)
    }
  }

    const executeOrganize = async () => {
      try {
        setError(null)
        setIsProcessing(true)
        setIsOrganizing(true)
        setOrganizeProgress(prev => ({ ...prev, status: 'Running' }))
        
        // 1. Start the process
        const response = await axios.post(`${API_BASE}/execute`, {
          sources: Array.from(selectedPaths)
        })
        
        if (response.data.status === 'success') {
         // 2. Setup SSE to listen for progress
         const eventSource = new EventSource(`${API_BASE}/organize-events`)
         
         eventSource.onmessage = (event) => {
           const progress = JSON.parse(event.data)
           setOrganizeProgress(progress)
           
           if (progress.status === 'Completed' || progress.status === 'Failed') {
             eventSource.close()
           }
         }

         eventSource.onerror = () => {
           eventSource.close()
           setIsOrganizing(false)
           setIsProcessing(false)
         }

         // 3. Wait for completion via polling or just wait for the SSE to finish
         // For simplicity in this implementation, we'll let the SSE handle the state
         // and then trigger the scan once we see 'Completed'.
         // However, since executeOrganize is an async function, we'll wrap the 
         // completion logic in a promise that resolves when status is 'Completed'
         
         await new Promise((resolve) => {
           const checkInterval = setInterval(() => {
             if (organizeProgressRef.current.status === 'Completed' || organizeProgressRef.current.status === 'Failed') {
               clearInterval(checkInterval)
               resolve(true)
             }
           }, 500)
         })

         // 4. Cleanup and Trigger scan
         setPreviewMoves([])
         await triggerScan()
         setSuccessMessage(`Successfully moved files!`)
         setIsSuccessDialogOpen(true)
         setIsOrganizing(false)
         eventSource.close()
       } else {
         setError(response.data.message)
         setIsProcessing(false)
         setIsOrganizing(false)
       }
     } catch (err) {
       setError('Failed to execute organization')
       setIsProcessing(false)
       setIsOrganizing(false)
     }
   }
 
   useEffect(() => {
     fetchMedia()
     const interval = setInterval(fetchStatus, 3000)
     const statusInterval = setInterval(fetchOrganizeStatus, 3000) // For persistence on refresh
     
     return () => {
       clearInterval(interval)
       clearInterval(statusInterval)
     }
   }, [])

  return (
    <div className="min-h-screen pb-20">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 pt-12">
         <Header 
           isScanning={isScanning} 
           isProcessing={isProcessing} 
           onTriggerScan={triggerScan} 
           onFetchPreview={fetchPreview}
           mediaCount={media.length}
         />
 
         {error && (
          <div className="glass-card border-red-500/50 text-red-400 px-6 py-4 rounded-2xl mb-12 flex items-center gap-4 animate-in fade-in slide-in-from-top-4">
            <span className="text-2xl">⚠️</span>
            <p className="font-semibold text-lg">{error}</p>
          </div>
        )}

          <PreviewSection 
            previewMoves={previewMoves} 
            selectedPaths={selectedPaths}
            onToggleSelect={(path) => {
              const newSelection = new Set(selectedPaths)
              if (newSelection.has(path)) {
                newSelection.delete(path)
              } else {
                newSelection.add(path)
              }
              setSelectedPaths(newSelection)
            }}
            onSelectAll={(isSelected) => {
              if (isSelected) {
                setSelectedPaths(new Set(previewMoves.map(m => m.source.replace(/\\/g, '/'))))
              } else {
                setSelectedPaths(new Set())
              }
            }}
            onToggleGroupSelect={(paths, isSelected) => {
              const newSelection = new Set(selectedPaths)
              paths.forEach(path => {
                if (isSelected) {
                  newSelection.add(path)
                } else {
                  newSelection.delete(path)
                }
              })
              setSelectedPaths(newSelection)
            }}
            isProcessing={isProcessing} 
            onExecute={executeOrganize} 
            organizeProgress={organizeProgress}
          />

          <LibraryExplorer 
            media={media} 
            filter={filter} 
            setFilter={setFilter} 
          />
        </div>

        <ProgressDialog
          isOpen={isOrganizing}
          title="Organizing Media"
          message="Reorganizing your library into the Jellyfin structure..."
          progress={organizeProgress}
        />
  
        <SuccessDialog
        isOpen={isSuccessDialogOpen}
        title="Organization Complete"
        message={successMessage}
        onConfirm={() => setIsSuccessDialogOpen(false)}
      />
    </div>
  )
}

export default App