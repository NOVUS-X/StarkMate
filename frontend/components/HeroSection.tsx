"use client";
import React, { useEffect, useState } from "react";
import { FaArrowDown } from "react-icons/fa"; // Import the arrow icon

import Link from "next/link";
import Chessboard from "./Chessboard";

const HeroSection = () => {
  const [isAnimating, setIsAnimating] = useState(false);

  useEffect(() => {
    setIsAnimating(true);

    // Animation cycling
    const interval = setInterval(() => {
      setIsAnimating(false);
      setTimeout(() => setIsAnimating(true), 100);
    }, 10000);

    return () => clearInterval(interval);
  }, []);

  const scrollToFooter = () => {
    const footer = document.getElementById("footer");
    if (footer) {
      window.scrollTo({
        top: footer.offsetTop,
        behavior: "smooth",
      });
    }
  };

  return (
    <div className="relative min-h-screen overflow-hidden bg-gradient-to-br from-gray-900 via-indigo-950 to-purple-900">
      {/* Blockchain grid background */}
      <div className="absolute inset-0 opacity-20 grid grid-cols-12 grid-rows-12">
        {Array.from({ length: 144 }, (_, i) => (
          <div
            key={i}
            className="jsx-51939d8b18ab707d border border-cyan-500/30 bg-cyan-500/10"
          />
        ))}
      </div>

      {/* Animated particles */}
      <div className="absolute inset-0 z-0">
        {Array(20)
          // @ts-ignore
          .fill()
          .map((_, i) => (
            <div
              key={i}
              className={`absolute w-2 h-2 rounded-full bg-cyan-400 opacity-80 ${
                isAnimating ? "animate-float" : ""
              }`}
              style={{
                left: `${Math.random() * 100}%`,
                top: `${Math.random() * 100}%`,
                animationDelay: `${Math.random() * 5}s`,
                animationDuration: `${5 + Math.random() * 10}s`,
              }}
            />
          ))}
      </div>

      {/* Content container */}
      <div className="relative z-10 container mx-auto px-4 h-screen flex flex-col md:flex-row items-center justify-center md:justify-between">
        {/* Text content */}
        <div className="w-full md:w-1/2 text-center md:text-left mb-12 md:mb-0">
          <h1 className="text-4xl md:text-6xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-400 mb-6 tracking-tight">
            <span className="">STARK</span>
            <span className="">MATE</span>
          </h1>

          <p className="text-gray-300 text-xl md:text-2xl mb-8 max-w-xl">
            Play. Compete. Collect.{" "}
            <span className="text-cyan-400">On-Chain</span>.
            <br />
            <span className="text-lg">Every move secured by Starknet.</span>
          </p>

          <div className="flex flex-col sm:flex-row items-center justify-center md:justify-start space-y-4 sm:space-y-0 sm:space-x-4">
            <Link href="/play">
              <button className="px-8 py-3 rounded-full bg-gradient-to-r from-cyan-500 to-blue-600 text-white font-semibold text-lg hover:from-cyan-600 hover:to-blue-700 transition-all duration-300 shadow-lg hover:shadow-cyan-500/50 w-48">
                Play Now
              </button>
            </Link>

            <button className=" py-3 rounded-full bg-transparent border-2 border-purple-500 text-white font-semibold text-lg hover:bg-purple-500/20 transition-all duration-300 w-48">
              Connect Wallet
            </button>
          </div>
        </div>

        {/* Visual content */}
        <div className="w-full md:w-1/2 flex justify-center ">
          <div className="relative w-72 h-72 md:w-96 md:h-96">
            {/* Chess board with blockchain elements */}
            <div className="absolute inset-0 rotate-45 ">
              <Chessboard />
            </div>

            {/* Circle of small blockchain nodes */}
            {Array(12)
              .fill(null)
              .map((_, i) => (
                <div
                  key={i}
                  className="absolute w-3 h-3 md:w-4 md:h-4 bg-purple-500 rounded-full shadow-lg shadow-purple-500/50"
                  style={{
                    left: `${50 + 45 * Math.cos((i * Math.PI) / 6)}%`,
                    top: `${50 + 45 * Math.sin((i * Math.PI) / 6)}%`,
                    animation: `pulse ${2 + (i % 3)}s infinite ${i * 0.2}s`,
                  }}
                />
              ))}

            {/* Connection lines */}
            <svg
              className="absolute inset-0 w-full h-full"
              viewBox="0 0 100 100"
            >
              <g className="opacity-60">
                {Array(6)
                  .fill(null)
                  .map((_, i) => (
                    <line
                      key={i}
                      x1={50 + 45 * Math.cos((i * Math.PI) / 3)}
                      y1={50 + 45 * Math.sin((i * Math.PI) / 3)}
                      x2={50 + 45 * Math.cos(((i + 3) * Math.PI) / 3)}
                      y2={50 + 45 * Math.sin(((i + 3) * Math.PI) / 3)}
                      stroke="rgb(139, 92, 246)"
                      strokeWidth="0.5"
                      className={isAnimating ? "animate-pulse" : ""}
                    />
                  ))}
              </g>
            </svg>
          </div>
        </div>
        {/* Scroll indicator */}
        <div
          onClick={scrollToFooter}
          className="absolute bottom-8 right-8 w-12 h-12 rounded-full bg-gradient-to-r from-cyan-500 to-purple-600 flex items-center justify-center cursor-pointer shadow-lg hover:shadow-cyan-500/50 transition-all duration-300 group"
          aria-label="Scroll to top"
        >
          <FaArrowDown className=" animate-bounce w-5 h-5 text-white group-hover:scale-110 transition-transform" />
        </div>
      </div>
    </div>
  );
};

export default HeroSection;
