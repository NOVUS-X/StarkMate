"use client";

import React, { useState, useEffect } from "react";
// Remove the useIsMobile import if not needed elsewhere
// import { useIsMobile } from "@/hook/use-mobile";
import Image from "next/image";

interface ChessboardComponentProps {
  position: string;
  onDrop: (params: { sourceSquare: string; targetSquare: string }) => void;
  width?: number;
}

const ChessboardComponent: React.FC<ChessboardComponentProps> = ({
  position,
  onDrop,
  width,
}) => {
  const [mounted, setMounted] = useState(false);
  const [boardState, setBoardState] = useState<string[][]>([]);
  const [boardWidth, setBoardWidth] = useState(width || 560);
  const [selectedSquare, setSelectedSquare] = useState<string | null>(null);
  // Remove the unused isMobile variable
  // const isMobile = useIsMobile();

  // Handle responsive sizing
  useEffect(() => {
    const updateBoardSize = () => {
      const container = document.querySelector(
        ".chessboard-container"
      )?.parentElement;
      if (!container) return;

      // Get the viewport width
      const vw = Math.max(
        document.documentElement.clientWidth || 0,
        window.innerWidth || 0
      );

      // Calculate the optimal board size
      const containerWidth = container.clientWidth;
      const maxSize = 560;
      const minSize = Math.min(320, containerWidth); // Ensure minimum size is not larger than container

      // Calculate new width based on screen size
      let newWidth;
      if (vw < 768) {
        // For mobile screens
        newWidth = Math.max(minSize, Math.min(containerWidth * 0.95, maxSize));
      } else {
        // For larger screens
        newWidth = Math.min(containerWidth, maxSize);
      }

      setBoardWidth(newWidth);
    };

    if (mounted) {
      updateBoardSize();
      window.addEventListener("resize", updateBoardSize);
      window.addEventListener("orientationchange", updateBoardSize);
    }

    return () => {
      window.removeEventListener("resize", updateBoardSize);
      window.removeEventListener("orientationchange", updateBoardSize);
    };
  }, [mounted]);

  // Only render the chessboard on the client side
  useEffect(() => {
    setMounted(true);

    // Parse FEN position string to create board state
    if (position === "start") {
      // Standard starting position
      setBoardState([
        ["bR", "bN", "bB", "bQ", "bK", "bB", "bN", "bR"],
        ["bP", "bP", "bP", "bP", "bP", "bP", "bP", "bP"],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["", "", "", "", "", "", "", ""],
        ["wP", "wP", "wP", "wP", "wP", "wP", "wP", "wP"],
        ["wR", "wN", "wB", "wQ", "wK", "wB", "wN", "wR"],
      ]);
    } else {
      // Try to parse custom FEN position
      try {
        const fenParts = position.split(" ");
        const rows = fenParts[0].split("/");
        const newBoard: string[][] = [];

        rows.forEach((row) => {
          const newRow: string[] = [];
          for (let i = 0; i < row.length; i++) {
            const char = row[i];
            if (isNaN(parseInt(char))) {
              // It's a piece
              const color = char === char.toUpperCase() ? "w" : "b";
              newRow.push(`${color}${char.toUpperCase()}`);
            } else {
              // It's a number (empty squares)
              for (let j = 0; j < parseInt(char); j++) {
                newRow.push("");
              }
            }
          }
          newBoard.push(newRow);
        });

        setBoardState(newBoard);
      } catch (e) {
        console.error("Error parsing FEN:", e);
        // Fallback to empty board
        setBoardState(Array(8).fill(Array(8).fill("")));
      }
    }
  }, [position]);

  // Get piece image based on piece code
  const getPieceImage = (piece: string) => {
    if (!piece) return null;

    // Map piece codes to image paths
    const pieceImages: Record<string, string> = {
      wP: "/images/chess/wp.png",
      wR: "/images/chess/wr.png",
      wN: "/images/chess/wn.png",
      wB: "/images/chess/wb.png",
      wQ: "/images/chess/wq.png",
      wK: "/images/chess/wk.png",
      bP: "/images/chess/bp.png",
      bR: "/images/chess/br.png",
      bN: "/images/chess/bn.png",
      bB: "/images/chess/bb.png",
      bQ: "/images/chess/bq.png",
      bK: "/images/chess/bk.png",
    };

    // Fallback to Unicode symbols if images aren't available
    const pieceSymbols: Record<string, string> = {
      wP: "♙",
      wR: "♖",
      wN: "♘",
      wB: "♗",
      wQ: "♕",
      wK: "♔",
      bP: "♟",
      bR: "♜",
      bN: "♞",
      bB: "♝",
      bQ: "♛",
      bK: "♚",
    };

    const isWhite = piece.startsWith("w");
    const squareSize = boardWidth / 8; // This now has access to boardWidth from component state

    return (
      <div
        className="piece-container"
        style={{
          width: "100%",
          height: "100%",
          display: "flex",
          justifyContent: "center",
          alignItems: "center",
          position: "relative",
          userSelect: "none",
          cursor: "grab",
          transform: `scale(${boardWidth < 400 ? 0.7 : 0.9})`, // Reduce scale on smaller screens
          transition: "transform 0.2s ease",
        }}
      >
        <div
          style={{
            width: boardWidth < 400 ? "80%" : "90%", // Reduce container size on smaller screens
            height: boardWidth < 400 ? "80%" : "90%",
            position: "relative",
          }}
        >
          <Image
            src={pieceImages[piece]}
            alt={piece}
            fill
            priority
            sizes={`(max-width: 768px) ${squareSize}px, ${squareSize}px`}
            style={{
              objectFit: "contain",
              filter: isWhite
                ? "brightness(0.85) sepia(0.3) hue-rotate(170deg) saturate(0.8) drop-shadow(2px 2px 2px rgba(0,0,0,0.5))"
                : "drop-shadow(2px 2px 2px rgba(0,0,0,0.3))",
              willChange: "transform",
            }}
            onError={(e) => {
              e.currentTarget.style.display = "none";
              const parent = e.currentTarget.parentElement;
              if (parent) {
                parent.innerHTML = `<div style="font-size: ${
                  squareSize * (boardWidth < 400 ? 0.5 : 0.7) // Reduce font size for Unicode fallback on smaller screens
                }px; color: ${isWhite ? "#005dad" : "#333333"}; text-shadow: ${
                  isWhite
                    ? "0px 1px 2px rgba(0,0,0,0.7), -1px -1px 2px rgba(0,0,0,0.7)"
                    : "0px 1px 2px rgba(0,0,0,0.5)"
                };">${pieceSymbols[piece]}</div>`;
              }
            }}
          />
        </div>
      </div>
    );
  };

  // Handle click for mobile or touch devices
  const handleSquareClick = (row: number, col: number) => {
    const clickedSquare = `${row},${col}`;

    // If no square is selected yet, select this one if it has a piece
    if (!selectedSquare && boardState[row][col]) {
      setSelectedSquare(clickedSquare);
      return;
    }

    // If this is the same square that was already selected, deselect it
    if (selectedSquare === clickedSquare) {
      setSelectedSquare(null);
      return;
    }

    // If another square was already selected, try to make a move
    if (selectedSquare) {
      const [sourceRow, sourceCol] = selectedSquare.split(",").map(Number);

      // Convert to algebraic notation
      const sourceSquare = `${String.fromCharCode(97 + sourceCol)}${
        8 - sourceRow
      }`;
      const targetSquare = `${String.fromCharCode(97 + col)}${8 - row}`;

      // Call the onDrop callback
      onDrop({ sourceSquare, targetSquare });

      // Reset selection
      setSelectedSquare(null);
    }
  };

  // Handle drag start
  const handleDragStart = (e: React.DragEvent, row: number, col: number) => {
    e.dataTransfer.setData("text/plain", `${row},${col}`);
    const draggedElement = e.currentTarget as HTMLElement;
    if (draggedElement) {
      draggedElement.style.opacity = "0.6";
    }
  };

  // Handle drag end
  const handleDragEnd = (e: React.DragEvent) => {
    const draggedElement = e.currentTarget as HTMLElement;
    if (draggedElement) {
      draggedElement.style.opacity = "1";
    }
  };

  // Handle drop
  const handleDrop = (
    e: React.DragEvent,
    targetRow: number,
    targetCol: number
  ) => {
    e.preventDefault();
    const data = e.dataTransfer.getData("text/plain");
    const [sourceRow, sourceCol] = data.split(",").map(Number);

    // Convert to algebraic notation
    const sourceSquare = `${String.fromCharCode(97 + sourceCol)}${
      8 - sourceRow
    }`;
    const targetSquare = `${String.fromCharCode(97 + targetCol)}${
      8 - targetRow
    }`;

    // Call the onDrop callback and update UI immediately
    const moveSuccess = onDrop({ sourceSquare, targetSquare });
    if (moveSuccess) {
      // Force a re-render to update piece positions
      setSelectedSquare(null);
    }
  };

  // Handle drag over
  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
  };

  // If not mounted yet, show a placeholder
  if (!mounted) {
    return (
      <div className="w-full h-full flex items-center justify-center bg-gray-800 rounded-md">
        <div className="text-white">Initializing chessboard...</div>
      </div>
    );
  }

  return (
    <div
      className="chessboard-container w-full mx-auto relative"
      style={{
        width: "100%",
        maxWidth: `${boardWidth}px`,
        minWidth: "320px",
        aspectRatio: "1/1",
        display: "grid",
        gridTemplateColumns: `repeat(8, minmax(0, 1fr))`,
        gridTemplateRows: `repeat(8, minmax(0, 1fr))`,
        border: "2px solid #005dad",
        borderRadius: "4px",
        boxShadow: "0 8px 16px rgba(0, 93, 173, 0.3)",
        overflow: "visible",
        touchAction: "none",
        margin: "0 auto",
        padding: "1%",
        transform: "scale(var(--board-scale, 1))", // Add transform scale
        transformOrigin: "center center",
      }}
    >
      {boardState.map((row, rowIndex) =>
        row.map((piece, colIndex) => {
          const isLight = (rowIndex + colIndex) % 2 === 1;
          const isSelected = selectedSquare === `${rowIndex},${colIndex}`;

          return (
            <div
              key={`${rowIndex}-${colIndex}`}
              style={{
                backgroundColor: isLight ? "#008e90" : "#ffffff", // Teal for dark squares, white for light squares
                width: "100%",
                height: "100%",
                display: "flex",
                justifyContent: "center",
                alignItems: "center",
                cursor: piece ? "grab" : "default",
                position: "relative",
                boxShadow: isSelected
                  ? "inset 0 0 0 3px rgba(0, 93, 173, 0.75)" // Blue highlight for selected squares
                  : "none",
                transition: "background-color 0.2s ease",
              }}
              onClick={() => handleSquareClick(rowIndex, colIndex)}
              draggable={!!piece}
              onDragStart={(e) => handleDragStart(e, rowIndex, colIndex)}
              onDragEnd={handleDragEnd}
              onDrop={(e) => handleDrop(e, rowIndex, colIndex)}
              onDragOver={handleDragOver}
            >
              {piece && (
                <div
                  style={{
                    transition: "transform 0.2s ease-out", // Add smooth transition for piece movement
                    transform: `scale(${isSelected ? 1.1 : 1})`, // Add subtle scaling effect for selected pieces
                  }}
                >
                  {getPieceImage(piece)}
                </div>
              )}
            </div>
          );
        })
      )}
    </div>
  );
};

export default ChessboardComponent;
