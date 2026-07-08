/**
 * SuccessDialog component for beautiful, consistent success notifications.
 * Uses Tailwind CSS for a glassmorphism aesthetic matching the app.
 */
import React from 'react';

const SuccessDialog = ({ isOpen, title, message, onConfirm }) => {
  if (!isOpen) return null;

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center p-4 sm:p-6">
      {/* Backdrop */}
      <div 
        className="absolute inset-0 bg-black/60 backdrop-blur-sm animate-in fade-in duration-300"
        onClick={onConfirm}
      />
      
      {/* Dialog Card */}
      <div className="relative w-full max-w-md glass-card border border-white/10 shadow-2xl rounded-3xl overflow-hidden animate-in zoom-in-95 fade-in duration-300">
        {/* Accent Glow */}
        <div className="absolute -top-24 -right-24 w-48 h-48 bg-emerald-500/20 blur-[60px] pointer-events-none" />
        
        <div className="p-8">
          <div className="flex flex-col items-center text-center">
            <div className="w-16 h-16 bg-emerald-500/10 rounded-full flex items-center justify-center mb-4 text-3xl">
              ✅
            </div>
            <h3 className="text-2xl font-bold text-white mb-2">{title}</h3>
            <p className="text-gray-400 leading-relaxed mb-8">
              {message}
            </p>
          </div>

          <div className="flex flex-col gap-3">
            <button
              onClick={onConfirm}
              className="w-full px-6 py-3 rounded-xl btn-primary-purple font-semibold transition-all duration-200 shadow-lg shadow-purple-500/20"
            >
              Great!
            </button>
          </div>
        </div>
      </div>
    </div>
  );
};

export default SuccessDialog;