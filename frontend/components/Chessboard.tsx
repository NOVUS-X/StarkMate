import React from 'react';

const Chessboard = () => {
  // Create 8x8 board array
  const board = Array(8).fill(null).map((_, row) =>
    Array(8).fill(null).map((_, col) => (row + col) % 2 === 0)
  );

  return (
    <div className="relative w-[95%]  md:w-[100%]">
      {/* Glow effect behind the board */}
      <div className="absolute inset-0 bg-cyan-500/20 blur-xl rounded-xl"></div>
      
      <div className="relative grid grid-cols-8 w-full h-full border border-cyan-500/50 rounded-xl overflow-hidden">
        {board.map((row, i) =>
          row.map((isLight, j) => (
            <div
              key={`${i}-${j}`}
              className={`aspect-square ${
                isLight
                  ? 'bg-gray-700/80 '
                  : 'bg-gray-900/90'
              } transition-colors duration-300 `}
            />
          ))
        )}
      </div>
      
      {/* Overlay gradient */}
      <div className="absolute inset-0 bg-gradient-to-br from-transparent via-transparent to-purple-900/30 rounded-xl"></div>
    </div>
  );
};

export default Chessboard; 