/**
 * Main Application component for Jellynizer.
 * Coordinates state management and layout.
 */
import React, { useState, useEffect } from 'react'
import axios from 'axios'
import Header from './components/Header'
import LibraryExplorer from './components/LibraryExplorer'
import PreviewSection from './components/PreviewSection'
import ConfirmationDialog from './components/ConfirmationDialog'
import SuccessDialog from './components/SuccessDialog'

const API_BASE = import.meta.env.VITE_API_BASE_URL || '/api'

function App() {
  const [media, setMedia] = useState([])
  const [isScanning, setIsScanning] = useState(false)
  const [isProcessing, setIsProcessing] = useState(false)
  const [previewMoves, setPreviewMoves] = useState([])
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
      const response = await axios.post(`${API_BASE}/execute`)
      if (response.data.status === 'success') {
        setPreviewMoves([])
        await triggerScan()
        setSuccessMessage(`Successfully moved ${response.data.moved_count} files!`)
        setIsSuccessDialogOpen(true)
      } else {
        setError(response.data.message)
      }
    } catch (err) {
      setError('Failed to execute organization')
    } finally {
      setIsProcessing(false)
    }
  }

  useEffect(() => {
    fetchMedia()
    const interval = setInterval(fetchStatus, 3000)
    return () => clearInterval(interval)
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
          isProcessing={isProcessing} 
          onExecute={executeOrganize} 
        />

        <LibraryExplorer 
          media={media} 
          filter={filter} 
          setFilter={setFilter} 
        />
      </div>

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