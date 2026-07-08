/**
 * LibraryExplorer component.
 * Displays the list of media files with filtering capabilities.
 */
import React from 'react'
import MediaCard from './MediaCard'

/**
 * @param {Object} props
 * @param {Array} props.media - The list of media files.
 * @param {string} props.filter - The current filter ('all', 'movies', 'shows').
 * @param {Function} props.setFilter - Function to update the filter.
 */
function LibraryExplorer({ media, filter, setFilter }) {
  return (
    <div className="mt-12">
      <div className="flex flex-col sm:flex-row sm:items-center justify-between mb-8 gap-4">
        <h2 className="text-2xl font-bold text-white flex items-center gap-2">
          <span className="text-purple-400">📂</span> Library Explorer
        </h2>
        
        <div className="flex items-center gap-2">
          <div className="flex bg-white/5 p-1 rounded-xl border border-white/10">
            <button
              onClick={() => setFilter('all')}
              className={`px-4 py-1.5 rounded-lg text-sm font-medium transition-all ${
                filter === 'all' 
                ? 'bg-purple-600 text-white shadow-lg' 
                : 'text-gray-400 hover:text-gray-200'
              }`}
            >
              All
            </button>
            <button
              onClick={() => setFilter('movies')}
              className={`px-4 py-1.5 rounded-lg text-sm font-medium transition-all ${
                filter === 'movies' 
                ? 'bg-purple-600 text-white shadow-lg' 
                : 'text-gray-400 hover:text-gray-200'
              }`}
            >
              Movies
            </button>
            <button
              onClick={() => setFilter('shows')}
              className={`px-4 py-1.5 rounded-lg text-sm font-medium transition-all ${
                filter === 'shows' 
                ? 'bg-purple-600 text-white shadow-lg' 
                : 'text-gray-400 hover:text-gray-200'
              }`}
            >
              Shows
            </button>
          </div>

          <div className="text-sm text-gray-400 font-medium bg-white/5 px-4 py-1.5 rounded-full border border-white/5">
            {media.filter(f => 
              filter === 'all' || 
              (filter === 'movies' && f.media_type === 'Movie') || 
              (filter === 'shows' && f.media_type === 'TvShow')
            ).length} {media.length === 1 ? 'file' : 'files'} found
          </div>
        </div>
      </div>

      <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-6">
        {(() => {
          const filteredMedia = media.filter(file => {
            if (filter === 'movies') return file.media_type === 'Movie';
            if (filter === 'shows') return file.media_type === 'TvShow';
            return true;
          });

          if (filteredMedia.length === 0) {
            return (
              <div className="col-span-full flex flex-col items-center justify-center py-40 text-gray-500 glass-card rounded-3xl border-dashed border-2 border-white/5">
                <div className="text-7xl mb-6 opacity-20">📂</div>
                <p className="text-2xl font-light text-gray-300 mb-2">No media files discovered.</p>
                <p className="text-sm opacity-50">Try a different filter or start a new scan.</p>
              </div>
            );
          }

          return filteredMedia.map((file, idx) => (
            <MediaCard key={idx} file={file} />
          ));
        })()}
      </div>
    </div>
  )
}

export default LibraryExplorer