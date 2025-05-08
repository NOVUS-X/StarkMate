import React from 'react';

export default function DotPulseLoader() {
    return (
        <div className="flex space-x-1">
            <div className="w-2 h-2 bg-teal-500 rounded-full animate-pulse" />
            <div className="w-2 h-2 bg-teal-500 rounded-full animate-pulse delay-100" />
            <div className="w-2 h-2 bg-teal-500 rounded-full animate-pulse delay-200" />
        </div>
    );
} 