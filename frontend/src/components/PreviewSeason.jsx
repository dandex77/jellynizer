/**
 * PreviewSeason component to display a single season's moves.
 * @param {Object} props
 * @param {string} props.seasonName - The name of the season (e.g., "Season 01").
 * @param {Array} props.moves - The list of move objects in this season.
 * @param {Set} props.selectedPaths - The set of currently selected paths.
 * @param {Function} props.onToggleSelect - Callback to toggle selection for a path.
 * @param {Function} props.onToggleGroupSelect - Callback to toggle selection for a group of paths.
 * @param {boolean} props.isExpanded - Whether this season is expanded.
 * @param {Function} props.onToggleExpand - Callback to toggle expansion.
 * @param {boolean} props.isProcessing - Whether the overall organization is in progress.
 */
import React from 'react';
import PreviewItem from './PreviewItem';

const PreviewSeason = ({ 
  seasonName, 
  moves, 
  selectedPaths, 
  onToggleSelect, 
  onToggleGroupSelect,
  isExpanded, 
  onToggleExpand,
  isProcessing,
  seasonGroupKey
}) => {
  const seasonPaths = moves.map(m => m.source.replace(/\\/g, '/'));
  const isSeasonSelected = seasonPaths.length > 0 && seasonPaths.every(p => selectedPaths.has(p));

  return (
    <div className="space-y-3">
      <div 
        className="flex items-center justify-between cursor-pointer group/season"
        onClick={() => onToggleExpand(seasonGroupKey)}
      >
        <div className="flex items-center gap-2">
          <span className="text-xs text-gray-500 group-hover/season:text-gray-400">
            {isExpanded ? '▾' : '▸'}
          </span>
          <span className="text-sm font-medium text-gray-400">{seasonName}</span>
        </div>
        
        <div className="flex items-center gap-2">
          <input 
            type="checkbox"
            checked={isSeasonSelected}
            onChange={(e) => {
              e.stopPropagation();
              onToggleGroupSelect(seasonPaths, e.target.checked);
            }}
            className="w-3.5 h-3.5 rounded border-white/10 bg-white/5 text-purple-500 focus:ring-purple-500 focus:ring-offset-0 cursor-pointer"
            disabled={isProcessing}
          />
          <span className="text-[10px] text-gray-600 uppercase font-bold">Select Season</span>
        </div>
      </div>

      {isExpanded && (
        <div className="grid gap-3 ml-4 pl-4 border-l border-white/5">
          {moves.map((move) => (
            <PreviewItem
              key={move.source}
              move={move}
              isSelected={selectedPaths.has(move.source.replace(/\\/g, '/'))}
              onSelect={onToggleSelect}
              isProcessing={isProcessing}
            />
          ))}
        </div>
      )}
    </div>
  );
};

export default PreviewSeason;