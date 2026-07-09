/**
 * ProgressDialog component to show a modal during long-running operations.
 */
import React from 'react';

const ProgressDialog = ({ isOpen, title, message, progress }) => {
  if (!isOpen) return null;

  const { current, total, current_file } = progress || {};
  const progressPercentage = total > 0 ? Math.round((current / total) * 100) : 0;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 bg-black/60 backdrop-blur-sm animate-in fade-in">
      <div className="glass-card w-full max-w-md overflow-hidden border border-white/10 shadow-2xl animate-in zoom-in-95 duration-200">
        <div className="p-8 text-center">
          <div className="inline-flex items-center justify-center w-16 h-16 rounded-full bg-purple-500/20 mb-6">
            <span className="text-3xl animate-pulse">⏳</span>
          </div>
          
          <h2 className="text-2xl font-bold text-white mb-2">{title}</h2>
          <p className="text-gray-400 mb-8">{message}</p>

          <div className="space-y-4">
            <div className="flex justify-between text-sm font-mono text-purple-400">
              <span>{progressPercentage}%</span>
              <span>{current} / {total}</span>
            </div>
            
            <div className="h-3 w-full bg-white/5 rounded-full overflow-hidden">
              <div 
                className="h-full bg-gradient-to-r from-purple-600 to-violet-500 transition-all duration-300 ease-out"
                style={{ width: `${progressPercentage}%` }}
              />
            </div>

            <p className="text-xs text-gray-500 truncate font-mono mt-4 px-2">
              {current_file || 'Initializing...'}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ProgressDialog;