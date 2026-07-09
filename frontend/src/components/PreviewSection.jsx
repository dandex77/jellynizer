/**
 * PreviewSection component to display proposed file organization changes.
 */
import React, { useState } from 'react';
import ConfirmationDialog from './ConfirmationDialog';

const PreviewSection = ({ previewMoves, isProcessing, onExecute, organizeProgress }) => {
  const [isDialogOpen, setIsDialogOpen] = useState(false);

  if (previewMoves.length === 0) return null;

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
          </div>
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
                disabled={isProcessing}
                className={`btn-primary-purple w-full lg:w-auto min-w-[200px] ${
                  isProcessing ? 'opacity-50 cursor-not-allowed' : ''
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
        </div>

        <div className="relative px-8 pb-8 space-y-4">
          <div className="grid gap-4">
            {previewMoves.map((move, idx) => (
              <div 
                key={idx} 
                className="flex items-center gap-4 text-sm bg-white/5 p-4 rounded-xl border border-white/5 hover:border-purple-500/30 hover:bg-white/10 transition-all duration-300 group"
              >
                <span className="text-purple-500/50 font-mono text-xs min-w-[24px]">
                  {String(idx + 1).padStart(2, '0')}
                </span>
                <div className="flex-1 min-w-0 flex flex-col gap-1">
                  <p className="text-blue-400 truncate font-mono text-xs sm:text-sm group-hover:text-blue-300">
                    {move.source.split(/[/\\]/).pop()}
                  </p>
                 <div className="flex items-center gap-2">
                   <span className="text-purple-500/40 text-xs">➞</span>
                   <div className="flex flex-col min-w-0">
                     <p className="text-emerald-400 truncate font-mono text-xs sm:text-sm group-hover:text-emerald-300">
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
            ))}
          </div>
          
          <div className="pt-6 mt-6 border-t border-white/5 text-center">
            <p className="text-gray-400 font-medium text-lg">
              Ready to organize <span className="text-purple-400 font-bold">{previewMoves.length}</span> files.
            </p>
          </div>
        </div>
      </div>

      <ConfirmationDialog
        isOpen={isDialogOpen}
        title="Confirm Organization"
        message={`Are you sure you want to reorganize ${previewMoves.length} files? This action will move files to their new locations.`}
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