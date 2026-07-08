/**
 * ConfirmationDialog component for beautiful, consistent user prompts.
 * Uses Tailwind CSS for a glassmorphism aesthetic matching the app.
 */
import React from 'react';

const ConfirmationDialog = ({ isOpen, title, message, onConfirm, onCancel }) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6">
      {/* Backdrop */}
      <div 
        className="absolute inset-0 bg-black/60 backdrop-blur-sm animate-in fade-in duration-300"
        onClick={onCancel}
      />
      
      {/* Dialog Card */}
      <div className="relative w-full max-w-md glass-card border border-white/10 shadow-2xl rounded-3xl overflow-hidden animate-in zoom-in-95 fade-in duration-300">
        {/* Accent Glow */}
        <div className="absolute -top-24 -right-24 w-48 h-48 bg-purple-500/20 blur-[60px] pointer-events-none" />
        
        <div className="p-8">
          <div className="flex flex-col items-center text-center">
            <div className="w-16 h-16 bg-amber-500/10 rounded-full flex items-center justify-center mb-4 text-3xl">
              ⚠️
            </div>
            <h3 className="text-2xl font-bold text-white mb-2">{title}</h3>
            <p className="text-gray-400 leading-relaxed mb-8">
              {message}
            </p>
          </div>

          <div className="flex flex-col sm:flex-row gap-3">
            <button
              onClick={onCancel}
              className="flex-1 px-6 py-3 rounded-xl bg-white/5 hover:bg-white/10 text-gray-300 font-semibold transition-all duration-200"
            >
              Cancel
            </button>
            <button
              onClick={onConfirm}
              className="flex-1 px-6 py-3 rounded-xl btn-primary-purple font-semibold transition-all duration-200 shadow-lg shadow-purple-500/20"
            >
              Confirm
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default ConfirmationDialog;