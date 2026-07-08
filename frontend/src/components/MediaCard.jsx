/**
 * MediaCard component to display individual media file information.
 */
import React from 'react';

const MediaCard = ({ file }) => {
  const isMovie = file.media_type === 'Movie';
  
  return (
    <div className="glass-card p-5 hover:border-purple-500/40 hover:shadow-purple-500/10 cursor-default group transition-all duration-300">
      <div className="flex justify-between items-start mb-4">
        <span className={`text-[10px] font-bold px-3 py-1 rounded-full uppercase tracking-wider ${
          isMovie ? 'bg-purple-500/20 text-purple-400' : 'bg-blue-500/20 text-blue-400'
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
        
        <div className="flex items-start gap-2 pt-1">
          <span className="text-xs opacity-50">📂</span>
          <p className="truncate text-xs opacity-50 font-mono leading-relaxed break-all">{file.path}</p>
        </div>
      </div>
    </div>
  );
};

export default MediaCard;