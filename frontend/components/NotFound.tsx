"use client";
import React from "react";
import Link from "next/link";
import Image from "next/image";
import chessking from "@/app/assets/chessking.jpeg";

const NotFound = () => {
	return (
		<div className="relative min-h-screen flex flex-col items-center justify-center bg-gradient-to-br from-gray-900 via-indigo-950 to-purple-900 text-white overflow-hidden">
			{/* Background Grid */}
			<div className="absolute inset-0 opacity-20 grid grid-cols-12 grid-rows-12">
				{Array.from({ length: 144 }, (_, i) => (
					<div
						key={i}
						className={`border border-cyan-500/30 bg-cyan-500/10 ${
							Math.random() > 0.92 ? "bg-cyan-500/20" : ""
						}`}
					/>
				))}
			</div>

			{/* Floating Particles */}
			{Array.from({ length: 30 }, (_, i) => (
				<div
					key={i}
					className="absolute w-2 h-2 rounded-full bg-cyan-400 opacity-70 animate-float"
					style={{
						left: `${Math.random() * 100}%`,
						top: `${Math.random() * 100}%`,
						animationDelay: `${Math.random() * 5}s`,
						animationDuration: `${5 + Math.random() * 10}s`,
					}}
				/>
			))}

			{/* Animated Floating Shapes */}
			{Array.from({ length: 10 }, (_, i) => {
				const size = Math.random() * 40 + 20; // Random size between 20-60px
				const left = Math.random() * 100;
				const top = Math.random() * 100;
				const animationDelay = Math.random() * 5;
				const animationDuration = Math.random() * 15 + 5;
				const rotation = Math.random() * 360;
				const shapeTypes = [
					"rounded-lg",
					"rounded-full",
					"clip-triangle",
				];
				const shape =
					shapeTypes[Math.floor(Math.random() * shapeTypes.length)];

				return (
					<div
						key={i}
						className={`absolute ${shape} bg-purple-400 opacity-60 animate-float-rotate`}
						style={{
							width: `${size}px`,
							height: `${size}px`,
							left: `${left}%`,
							top: `${top}%`,
							animationDelay: `${animationDelay}s`,
							animationDuration: `${animationDuration}s`,
							transform: `rotate(${rotation}deg)`,
						}}
					/>
				);
			})}

			{/* Content */}
			<div className="relative z-10 text-center">
				<h1 className="text-7xl font-bold text-transparent bg-clip-text bg-gradient-to-r from-cyan-400 to-purple-400 animate-glitch">
					404
				</h1>
				<p className="text-gray-300 text-xl mt-4">
					Oops! The page you're looking for doesn't exist.
				</p>

				{/* Floating Chess Image with Glowing Orbit */}
				<div className="relative w-60 h-60 mx-auto mt-6 flex items-center justify-center animate-float">
					{/* Glowing Orbit Ring */}
					<div className="absolute inset-0 rounded-full border-2 border-cyan-400/30 animate-spin-slow"></div>

					{/* Soft Glowing Aura */}
					<div className="absolute inset-0 rounded-full bg-gradient-to-br from-cyan-400/20 to-purple-500/20 blur-xl opacity-60 animate-pulse"></div>

					{/* Chess Image */}
					<Image
						src={chessking}
						alt="Chess King"
						width={220}
						height={220}
						className="relative rounded-lg shadow-lg shadow-cyan-500/40 transition-all duration-500 hover:scale-110 hover:shadow-purple-500/50"
					/>
				</div>

				{/* Buttons */}
				<div className="mt-8">
					<Link href="/">
						<button className="px-6 py-3 bg-gradient-to-r from-cyan-500 to-blue-600 text-white font-semibold text-lg rounded-full hover:from-cyan-600 hover:to-blue-700 transition-all duration-300 shadow-lg hover:shadow-cyan-500/50">
							Return Home
						</button>
					</Link>
				</div>
			</div>

			{/* Tailwind Animations */}
			<style jsx>{`
				@keyframes float {
					0% {
						transform: translateY(0px);
					}
					50% {
						transform: translateY(-20px);
					}
					100% {
						transform: translateY(0px);
					}
				}

				@keyframes float-rotate {
					0% {
						transform: rotate(0deg) translateY(0px);
					}
					50% {
						transform: rotate(180deg) translateY(-20px);
					}
					100% {
						transform: rotate(360deg) translateY(0px);
					}
				}

				@keyframes spin-slow {
					from {
						transform: rotate(0deg);
					}
					to {
						transform: rotate(360deg);
					}
				}

				.animate-float {
					animation: float 6s infinite ease-in-out;
				}

				.animate-float-rotate {
					animation: float-rotate 10s infinite ease-in-out;
				}

				.animate-spin-slow {
					animation: spin-slow 20s infinite linear;
				}
			`}</style>
		</div>
	);
};

export default NotFound;
