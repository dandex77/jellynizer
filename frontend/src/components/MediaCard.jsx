/**
 * MediaCard component to display individual media file information.
 */
import { useState } from 'react';

const MediaCard = ({ file, onCategorize }) => {
  const [isCategorizing, setIsCategorizing] = useState(false)
  const [mediaType, setMediaType] = useState('Movie')
  const [title, setTitle] = useState(
    file.title || file.path.split(/[\\/]/).pop()?.replace(/\.[^.]+$/, '') || ''
  )
  const [year, setYear] = useState(file.year?.toString() || '')
  const [season, setSeason] = useState('1')
  const [episode, setEpisode] = useState('1')
  const [error, setError] = useState('')
  const [isSaving, setIsSaving] = useState(false)
  const isMovie = file.media_type === 'Movie';
  const isUnknown = file.media_type === 'Unknown';

  const submitCategorization = async (event) => {
    event.preventDefault()
    setError('')
    setIsSaving(true)
    try {
      await onCategorize({
        path: file.path,
        media_type: mediaType,
        title,
        year: year ? Number(year) : null,
        season: mediaType === 'TvShow' ? Number(season) : null,
        episode: mediaType === 'TvShow' ? Number(episode) : null
      })
      setIsCategorizing(false)
    } catch (err) {
      setError(err.message || 'Failed to categorize media')
    } finally {
      setIsSaving(false)
    }
  }
  
  return (
    <div className="glass-card p-5 hover:border-purple-500/40 hover:shadow-purple-500/10 cursor-default group transition-all duration-300">
      <div className="flex justify-between items-start mb-4">
        <span className={`text-[10px] font-bold px-3 py-1 rounded-full uppercase tracking-wider ${
          isMovie ? 'bg-purple-500/20 text-purple-400' : isUnknown ? 'bg-gray-500/20 text-gray-300' : 'bg-blue-500/20 text-blue-400'
        }`}>
          {file.media_type}
        </span>
        <div className="opacity-0 group-hover:opacity-100 transition-opacity duration-300">
          <span className="text-purple-400 text-lg">✨</span>
        </div>
      </div>
      
      <h3 className="text-lg font-bold mb-2 text-gray-100 leading-tight truncate group-hover:text-purple-300 transition-colors" title={file.title}>
        {file.title || 'Unknown Title'}
      </h3>
      
       <div className="space-y-3 text-sm text-gray-400">
         <div className="flex flex-wrap items-center gap-x-2 gap-y-1">
           <div className="flex items-center gap-2">
             <span className="text-xs opacity-50">📅</span>
             <span className="font-medium">{file.year || 'N/A'}</span>
             {file.media_type === 'TvShow' && (
               <>
                 <span className="opacity-30">•</span>
                 <span className="font-medium text-xs">S{file.season} • E{file.episode}</span>
               </>
             )}
           </div>

           {file.identifiers && Object.keys(file.identifiers).length > 0 && (
             <div className="flex flex-wrap gap-1 ml-2">
               {Object.entries(file.identifiers).map(([provider, id]) => (
                 <span 
                   key={provider} 
                   className="text-[9px] px-1.5 py-0.5 rounded bg-purple-500/10 text-purple-400/80 border border-purple-500/20 font-mono"
                 >
                   {provider.toUpperCase()}: {id}
                 </span>
               ))}
             </div>
           )}
         </div>
         
         <div className="flex items-start gap-2 pt-1">
          <span className="text-xs opacity-50">📂</span>
          <p className="truncate text-xs opacity-50 font-mono leading-relaxed break-all">{file.path}</p>
        </div>
      </div>

      {isUnknown && (
        <button
          type="button"
          onClick={() => setIsCategorizing(true)}
          className="mt-4 w-full rounded-lg bg-purple-600/80 px-3 py-2 text-sm font-semibold text-white hover:bg-purple-500 transition-colors"
        >
          Categorize
        </button>
      )}

      {isCategorizing && (
        <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/70 p-4">
          <form
            role="dialog"
            aria-modal="true"
            aria-labelledby="categorize-title"
            onSubmit={submitCategorization}
            className="glass-card w-full max-w-lg space-y-5 rounded-2xl p-6"
          >
            <div>
              <h2 id="categorize-title" className="text-xl font-bold text-white">Categorize file</h2>
              <p className="mt-1 truncate text-sm text-gray-400" title={file.path}>{file.path}</p>
            </div>

            <fieldset>
              <legend className="mb-2 text-sm font-medium text-gray-200">Media type</legend>
              <div className="flex gap-2">
                {[
                  ['Movie', 'Movie'],
                  ['TvShow', 'TV show']
                ].map(([value, label]) => (
                  <button
                    key={value}
                    type="button"
                    onClick={() => setMediaType(value)}
                    aria-pressed={mediaType === value}
                    className={`rounded-lg px-4 py-2 text-sm font-medium ${
                      mediaType === value ? 'bg-purple-600 text-white' : 'bg-white/5 text-gray-300'
                    }`}
                  >
                    {label}
                  </button>
                ))}
              </div>
            </fieldset>

            <label className="block text-sm text-gray-200">
              Title
              <input
                autoFocus
                required
                value={title}
                onChange={event => setTitle(event.target.value)}
                className="mt-1 w-full rounded-lg border border-white/10 bg-black/30 px-3 py-2 text-white"
              />
            </label>

            <label className="block text-sm text-gray-200">
              Year <span className="text-gray-500">(optional)</span>
              <input
                type="number"
                min="1"
                value={year}
                onChange={event => setYear(event.target.value)}
                className="mt-1 w-full rounded-lg border border-white/10 bg-black/30 px-3 py-2 text-white"
              />
            </label>

            {mediaType === 'TvShow' && (
              <div className="grid grid-cols-2 gap-3">
                <label className="block text-sm text-gray-200">
                  Season
                  <input
                    type="number"
                    required
                    min="1"
                    value={season}
                    onChange={event => setSeason(event.target.value)}
                    className="mt-1 w-full rounded-lg border border-white/10 bg-black/30 px-3 py-2 text-white"
                  />
                </label>
                <label className="block text-sm text-gray-200">
                  Episode
                  <input
                    type="number"
                    required
                    min="1"
                    value={episode}
                    onChange={event => setEpisode(event.target.value)}
                    className="mt-1 w-full rounded-lg border border-white/10 bg-black/30 px-3 py-2 text-white"
                  />
                </label>
              </div>
            )}

            {error && <p role="alert" className="text-sm text-red-400">{error}</p>}

            <div className="flex justify-end gap-3">
              <button
                type="button"
                onClick={() => setIsCategorizing(false)}
                disabled={isSaving}
                className="rounded-lg px-4 py-2 text-sm text-gray-300 hover:bg-white/5"
              >
                Cancel
              </button>
              <button
                type="submit"
                disabled={isSaving}
                className="rounded-lg bg-purple-600 px-4 py-2 text-sm font-semibold text-white hover:bg-purple-500 disabled:opacity-50"
              >
                {isSaving ? 'Saving…' : 'Save category'}
              </button>
            </div>
          </form>
        </div>
      )}
    </div>
  );
};

export default MediaCard;