/**
 * PreviewItem component to display a single file move operation.
 * @param {Object} move - The move information.
 * @param {Function} onSelect - Callback to toggle selection for this item.
 * @param {boolean} isSelected - Whether this item is currently selected.
 * @param {boolean} isProcessing - Whether the overall organization is in progress.
 */
import React from 'react';

const PreviewItem = ({ move, onSelect, isSelected, isProcessing }) => {
  return (
    <div 
      className={`flex items-start gap-4 text-sm p-4 rounded-xl border transition-all duration-300 group/item ${
        isSelected
          ? 'bg-purple-500/10 border-purple-500/30'
          : 'bg-white/5 border-white/5 hover:border-purple-500/30 hover:bg-white/10'
      }`}
    >
      <input 
        type="checkbox"
        checked={isSelected}
        onChange={() => onSelect(move.source)}
        className="mt-1 w-4 h-4 rounded border-white/10 bg-white/5 text-purple-500 focus:ring-purple-500 focus:ring-offset-0 cursor-pointer"
        disabled={isProcessing}
      />
      
      <div className="flex-1 min-w-0 flex flex-col gap-1">
        <p className="text-blue-400 truncate font-mono text-xs sm:text-sm group-hover/item:text-blue-300">
          {move.source.split(/[/\\]/).pop()}
        </p>
        <div className="flex items-center gap-2">
          <span className="text-purple-500/40 text-xs">➞</span>
          <div className="flex flex-col min-w-0">
            <p className="text-emerald-400 truncate font-mono text-xs sm:text-sm group-hover/item:text-emerald-300">
              {move.destination.split(/[/\\]/).pop()}
            </p>
            <div className="flex flex-wrap gap-1 mt-1">
              {move.identifiers && Object.entries(move.identifiers).map(([provider, id]) => (
                <span 
                  key={provider} 
                  className="text-[8px] px-1 py-0 rounded bg-emerald-500/10 text-emerald-400/80 border border-emerald-500/20 font-mono"
                >
                  {provider.toUpperCase()}:{id}
                </span>
              ))}
            </div>
            <p className="text-gray-500 text-[10px] truncate font-mono">
              {move.relative_destination.split(/[/\\]/).join('/')}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default PreviewItem;