import React from 'react';

const ChessPieceSVG = ({ piece, color }: { piece: string; color: string }) => {
  const pieces: { [key: string]: string } = {
    pawn: 'M 20,42 L 20,35 C 20,33 18,31 18,29 C 18,27 22,27 22,23 C 22,21 20,19 20,19 C 20,19 25,18 25,14 C 25,11 22,10 22,10 L 28,10 C 28,10 25,11 25,14 C 25,18 30,19 30,19 C 30,19 28,21 28,23 C 28,27 32,27 32,29 C 32,31 30,33 30,35 L 30,42 L 20,42',
    rook: 'M 18,42 L 18,35 L 16,35 L 16,31 L 14,31 L 14,25 L 16,25 L 16,21 L 14,21 L 14,15 L 36,15 L 36,21 L 34,21 L 34,25 L 36,25 L 36,31 L 34,31 L 34,35 L 32,35 L 32,42 L 18,42',
    knight: 'M 20,42 L 20,35 C 20,35 18,34 18,32 C 18,30 18,28 19,27 C 20,26 21,23 21,21 C 21,19 20,18 20,16 C 20,14 22,13 22,13 C 25,13 28,15 30,17 C 32,19 34,21 34,25 C 34,29 31,31 31,31 L 31,35 L 29,35 L 29,42 L 20,42',
    bishop: 'M 20,42 L 20,35 C 20,33 18,31 18,29 C 18,27 22,27 22,23 C 22,21 20,19 20,19 C 20,19 25,18 25,14 C 25,11 22,10 22,10 L 28,10 C 28,10 25,11 25,14 C 25,18 30,19 30,19 C 30,19 28,21 28,23 C 28,27 32,27 32,29 C 32,31 30,33 30,35 L 30,42 L 20,42',
    queen: 'M 20,42 L 20,35 C 20,35 15,32 15,28 C 15,24 20,24 20,20 C 20,16 15,16 15,12 C 15,8 20,8 20,8 L 30,8 C 30,8 35,8 35,12 C 35,16 30,16 30,20 C 30,24 35,24 35,28 C 35,32 30,35 30,35 L 30,42 L 20,42',
    king: 'M 22.5,8 L 22.5,12 L 17.5,12 L 17.5,17 L 22.5,17 L 22.5,42 L 27.5,42 L 27.5,17 L 32.5,17 L 32.5,12 L 27.5,12 L 27.5,8 L 22.5,8'
  };

  return (
    <svg
      viewBox="0 0 50 50"
      className={`w-3/4 h-3/4 ${color === 'white' ? 'text-cyan-400/80' : 'text-purple-400/80'}`}
    >
      <path
        d={pieces[piece]}
        fill="currentColor"
        className="drop-shadow-[0_0_3px_rgba(0,255,255,0.5)]"
      />
    </svg>
  );
};

const initialBoard = [
  ['rook', 'knight', 'bishop', 'queen', 'king', 'bishop', 'knight', 'rook'],
  Array(8).fill('pawn'),
  Array(8).fill(''),
  Array(8).fill(''),
  Array(8).fill(''),
  Array(8).fill(''),
  Array(8).fill('pawn'),
  ['rook', 'knight', 'bishop', 'queen', 'king', 'bishop', 'knight', 'rook']
];

const Chessboard = () => {
  return (
    <div className="relative w-full h-full perspective-1000">
      {/* 3D glow effect behind the board */}
      <div className="absolute inset-0 bg-cyan-500/10 blur-2xl rounded-xl transform -rotate-6"></div>
      <div className="absolute inset-0 bg-purple-500/10 blur-2xl rounded-xl transform rotate-6"></div>
      
      <div className="relative grid grid-cols-8 w-full h-full border-2 border-cyan-500/30 rounded-xl overflow-hidden transform hover:scale-[1.02] transition-transform duration-500 shadow-[0_0_15px_rgba(34,211,238,0.2)]">
        {initialBoard.map((row, i) =>
          row.map((piece, j) => {
            const isLight = (i + j) % 2 === 0;
            return (
              <div
                key={`${i}-${j}`}
                className={`relative aspect-square ${
                  isLight
                    ? 'bg-gray-800/90'
                    : 'bg-gray-900/95'
                } transition-all duration-300 hover:bg-cyan-900/30 group`}
              >
                {/* Grid lines */}
                <div className="absolute inset-0 border-[0.5px] border-cyan-500/10"></div>
                
                {/* Piece */}
                {piece && (
                  <div className="absolute inset-0 flex items-center justify-center transform group-hover:scale-110 transition-transform duration-300">
                    <ChessPieceSVG
                      piece={piece}
                      color={i < 2 ? 'black' : 'white'}
                    />
                  </div>
                )}
                
                {/* Hover effect */}
                <div className="absolute inset-0 opacity-0 group-hover:opacity-100 bg-gradient-to-br from-cyan-500/5 to-purple-500/5 transition-opacity duration-300"></div>
              </div>
            );
          })
        )}
      </div>
      
      {/* Ambient light effect */}
      <div className="absolute inset-0 bg-gradient-to-br from-transparent via-transparent to-purple-900/20 rounded-xl pointer-events-none"></div>
    </div>
  );
};

export default Chessboard;