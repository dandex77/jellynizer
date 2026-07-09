/**
 * PreviewSection component to display proposed file organization changes.
 * Features: Grouped organization, collapsible sections, and group selection.
 */
import React, { useState, useMemo } from 'react';
import ConfirmationDialog from './ConfirmationDialog';
import PreviewGroup from './PreviewGroup';

const PreviewSection = ({ 
  previewMoves, 
  selectedPaths, 
  onToggleSelect, 
  onSelectAll, 
  onToggleGroupSelect,
  isProcessing, 
  onExecute, 
  organizeProgress 
}) => {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [expandedGroups, setExpandedGroups] = useState({}); // Key: title or title-season
  const [filter, setFilter] = useState('all'); // 'all', 'movies', 'shows'

  // Filtering logic
  const filteredMoves = useMemo(() => {
    return previewMoves.filter(move => {
      if (filter === 'movies') return move.media_type === 'Movie';
      if (filter === 'shows') return move.media_type === 'TvShow';
      return true;
    });
  }, [previewMoves, filter]);

  // Grouping logic
  const groupedMoves = useMemo(() => {
    const groups = {};

    filteredMoves.forEach(move => {
      const title = move.title || 'Unknown Title';
      const season = move.media_type === 'TvShow' ? move.season : null;

      if (!groups[title]) {
        groups[title] = {
          title,
          media_type: move.media_type,
          seasons: {}
        };
      }

      const seasonKey = season !== null ? `Season ${String(season).padStart(2, '0')}` : 'Movie/Show';
      if (!groups[title].seasons[seasonKey]) {
        groups[title].seasons[seasonKey] = [];
      }
      groups[title].seasons[seasonKey].push(move);
    });

    return Object.values(groups);
  }, [filteredMoves]);

  if (previewMoves.length === 0) return null;

  const toggleGroup = (groupKey) => {
    setExpandedGroups(prev => ({
      ...prev,
      [groupKey]: !prev[groupKey]
    }));
  };

  const isAllSelected = previewMoves.length > 0 && selectedPaths.size === previewMoves.length;

  const handleSelectAll = (e) => {
    onSelectAll(e.target.checked);
  };

  const getSelectedCount = () => selectedPaths.size;

  const showProgress = isProcessing && organizeProgress?.status === 'Running';
  const progressPercentage = organizeProgress?.total > 0 
    ? Math.round((organizeProgress.current / organizeProgress.total) * 100) 
    : 0;

  return (
    <>
      <div className="mb-16 glass-card overflow-hidden relative">
        {/* Subtle accent glow */}
        <div className="absolute -top-24 -right-24 w-64 h-64 bg-purple-500/10 blur-[100px] pointer-events-none" />
        <div className="absolute -bottom-24 -left-24 w-64 h-64 bg-violet-500/10 blur-[100px] pointer-events-none" />
        
        <div className="relative flex flex-col lg:flex-row justify-between items-center gap-8 p-8">
          <div className="text-center lg:text-left">
            <h2 className="text-3xl font-bold text-white flex items-center gap-3 justify-center lg:justify-start">
              <span className="text-purple-400 animate-pulse">✨</span> Proposed Changes
            </h2>
            <p className="text-gray-400 text-base mt-2">
              Review the reorganization plan before applying.
            </p>

            <div className="mt-6 flex items-center gap-2 justify-center lg:justify-start">
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
            </div>
          </div>
          <div className="w-full lg:w-auto flex flex-col items-center lg:items-end gap-6">
            <div className="w-full lg:w-auto">
              {showProgress ? (
                <div className="space-y-3 w-full lg:w-64">
                  <div className="flex justify-between text-xs text-purple-400 font-mono">
                    <span>{progressPercentage}%</span>
                    <span>{organizeProgress.current} / {organizeProgress.total}</span>
                  </div>
                  <div className="h-2 w-full bg-white/5 rounded-full overflow-hidden">
                    <div 
                      className="h-full bg-purple-500 transition-all duration-300 ease-out"
                      style={{ width: `${progressPercentage}%` }}
                    />
                  </div>
                  <p className="text-[10px] text-gray-500 truncate font-mono">
                    {organizeProgress.current_file || 'Processing...'}
                  </p>
                </div>
              ) : (
                <button
                  onClick={() => setIsDialogOpen(true)}
                  disabled={isProcessing || selectedPaths.size === 0}
                  className={`btn-primary-purple w-full lg:w-auto min-w-[200px] ${
                    isProcessing || selectedPaths.size === 0 ? 'opacity-50 cursor-not-allowed' : ''
                  }`}
                >
                  {isProcessing ? (
                    <>
                      <span className="animate-spin inline-block text-lg">⏳</span>
                      <span>Executing...</span>
                    </>
                  ) : (
                    'Confirm & Execute'
                  )}
                </button>
              )}
            </div>
            <div className="flex items-center gap-2 text-sm text-gray-400">
              <input 
                type="checkbox" 
                id="select-all"
                checked={isAllSelected}
                onChange={handleSelectAll}
                className="w-4 h-4 rounded border-white/10 bg-white/5 text-purple-500 focus:ring-purple-500 focus:ring-offset-0 cursor-pointer"
                disabled={isProcessing}
              />
              <label htmlFor="select-all" className="cursor-pointer select-none">Select All</label>
            </div>
          </div>
        </div>

        <div className="relative px-8 pb-8 space-y-4">
          <div className="space-y-6">
            {groupedMoves.map((group) => {
              const groupPaths = filteredMoves
                .filter(m => m.title === group.title)
                .map(m => m.source.replace(/\\/g, '/'));

              return (
                <PreviewGroup
                  key={group.title}
                  title={group.title}
                  seasons={group.seasons}
                  selectedPaths={selectedPaths}
                  onToggleSelect={onToggleSelect}
                  onToggleGroupSelect={onToggleGroupSelect}
                  isExpanded={expandedGroups[group.title]}
                  onToggleExpand={toggleGroup}
                  isProcessing={isProcessing}
                  groupPaths={groupPaths}
                  expandedGroups={expandedGroups}
                />
              );
            })}
          </div>
          
          <div className="pt-6 mt-6 border-t border-white/5 text-center">
            <p className="text-gray-400 font-medium text-lg">
              Ready to organize <span className="text-purple-400 font-bold">{selectedPaths.size}</span> files.
            </p>
          </div>
        </div>
      </div>

      <ConfirmationDialog
        isOpen={isDialogOpen}
        title="Confirm Organization"
        message={`Are you sure you want to reorganize ${getSelectedCount()} files? This action will move files to their new locations.`}
        onConfirm={async () => {
          setIsDialogOpen(false);
          await onExecute();
        }}
        onCancel={() => setIsDialogOpen(false)}
      />
    </>
  );
};

export default PreviewSection;