/**
 * PreviewGroup component to display a single movie/show group.
 * @param {Object} props
 * @param {string} props.title - The title of the movie/show.
 * @param {Object} props.seasons - An object containing the seasons and their moves.
 * @param {Set} props.selectedPaths - The set of currently selected paths.
 * @param {Function} props.onToggleSelect - Callback to toggle selection for a path.
 * @param {Function} props.onToggleGroupSelect - Callback to toggle selection for a group of paths.
 * @param {Function} props.onToggleExpand - Callback to toggle expansion of the group.
 * @param {boolean} props.isExpanded - Whether this group is expanded.
 * @param {boolean} props.isProcessing - Whether the overall organization is in progress.
 * @param {Array} props.groupPaths - The list of paths belonging to this group.
 * @param {Object} props.expandedGroups - The full expandedGroups state from the parent.
 */
import React from 'react';
import PreviewSeason from './PreviewSeason';

const PreviewGroup = ({ 
  title, 
  seasons, 
  selectedPaths, 
  onToggleSelect, 
  onToggleGroupSelect,
  isExpanded, 
  onToggleExpand,
  isProcessing,
  groupPaths,
  expandedGroups
}) => {
  const isGroupSelected = groupPaths.length > 0 && groupPaths.every(p => selectedPaths.has(p));

  return (
    <div className="bg-white/5 border border-white/10 rounded-2xl overflow-hidden">
      {/* Title Header */}
      <div className="flex items-center justify-between px-6 py-4 bg-white/5 border-b border-white/5">
        <div 
          className="flex items-center gap-3 cursor-pointer group/title"
          onClick={() => onToggleExpand(title)}
        >
          <span className="text-purple-500 group-hover/title:text-purple-400 transition-colors">
            {isExpanded ? '▾' : '▸'}
          </span>
          <h3 className="text-lg font-semibold text-purple-300">{title}</h3>
        </div>
        
        <div className="flex items-center gap-3">
          <input 
            type="checkbox"
            checked={isGroupSelected}
            onChange={(e) => onToggleGroupSelect(groupPaths, e.target.checked)}
            className="w-4 h-4 rounded border-white/10 bg-white/5 text-purple-500 focus:ring-purple-500 focus:ring-offset-0 cursor-pointer"
            disabled={isProcessing}
          />
          <span className="text-xs text-gray-500 font-medium uppercase tracking-wider">Select Group</span>
        </div>
      </div>

      {isExpanded && (
        <div className="p-6 space-y-6">
          {Object.entries(seasons).map(([seasonName, moves]) => {
            const seasonGroupKey = `${title}-${seasonName}`;
            return (
              <PreviewSeason
                key={seasonName}
                seasonName={seasonName}
                moves={moves}
                selectedPaths={selectedPaths}
                onToggleSelect={onToggleSelect}
                onToggleGroupSelect={onToggleGroupSelect}
                isExpanded={!!expandedGroups[seasonGroupKey]}
                onToggleExpand={onToggleExpand}
                isProcessing={isProcessing}
                seasonGroupKey={seasonGroupKey}
              />
            );
          })}
        </div>
      )}
    </div>
  );
};

export default PreviewGroup;