/**
 * Header component for the Jellynizer application.
 */
import React from 'react';

const Header = ({ isScanning, isProcessing, onTriggerScan, onFetchPreview, mediaCount }) => {
  return (
    <header className="flex flex-col lg:flex-row justify-between items-center gap-8 mb-16">
      <div className="text-center lg:text-left">
        <h1 className="text-6xl font-black purple-gradient-text tracking-tighter mb-2">
          Jellynizer
        </h1>
        <p className="text-gray-400 text-xl font-light tracking-wide opacity-80">
          Automated Media Organization Engine
        </p>
      </div>
      
      <div className="flex flex-col sm:flex-row items-stretch sm:items-center gap-4 w-full lg:w-auto">
        <button
          onClick={() => {
            console.log('Header button clicked');
            onTriggerScan();
          }}
          disabled={isScanning || isProcessing}
          className={`btn-primary-purple flex-1 sm:flex-none min-w-[180px] ${
            isScanning || isProcessing ? 'opacity-50 cursor-not-allowed' : ''
          }`}
        >
          {isScanning ? (
            <>
              <span className="animate-spin inline-block text-lg">🌀</span>
              <span>Scanning...</span>
            </>
          ) : (
            'Start New Scan'
          )}
        </button>

        {mediaCount > 0 && (
          <button
            onClick={onFetchPreview}
            disabled={isScanning || isProcessing}
            className="btn-primary-secondary flex-1 sm:flex-none min-w-[180px] ${
              isScanning || isProcessing ? 'opacity-50 cursor-not-allowed' : ''
            }"
          >
            {isProcessing ? (
              <>
                <span className="animate-spin inline-block text-lg">⏳</span>
                <span>Processing...</span>
              </>
            ) : (
              'Preview Changes'
            )}
          </button>
        )}
      </div>
    </header>
  );
};

export default Header;