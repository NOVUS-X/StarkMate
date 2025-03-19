"use client";

import React, { useState } from "react";
import {
  ChevronLeft,
  ChevronRight,
  Diamond,
  Award,
  Sparkles,
  Plus,
  Expand,
  Info,
} from "lucide-react";

const NFTGalleryPreview = () => {
  const [activeCategory, setActiveCategory] = useState("chess-pieces");

  // Sample NFT data - would come from your API or blockchain in a real app
  const nftCollections = {
    "chess-pieces": [
      {
        id: "nft-001",
        name: "Quantum King",
        rarity: "Legendary",
        owner: "CryptoChessmaster",
        price: "2.5 ETH",
        image: "/api/placeholder/300/300",
        description:
          "Limited edition king piece with quantum effects and special in-game abilities.",
      },
      {
        id: "nft-002",
        name: "Neural Queen",
        rarity: "Epic",
        owner: "StarknetWhiz",
        price: "1.8 ETH",
        image: "/api/placeholder/300/300",
        description:
          "AI-enhanced queen piece that adapts to your playing style.",
      },
      {
        id: "nft-003",
        name: "Blockchain Bishop",
        rarity: "Rare",
        owner: "ZKProofPlayer",
        price: "1.2 ETH",
        image: "/api/placeholder/300/300",
        description:
          "Special bishop piece that leaves data trails on the blockchain.",
      },
      {
        id: "nft-004",
        name: "Cyber Knight",
        rarity: "Rare",
        owner: "L2Enthusiast",
        price: "0.9 ETH",
        image: "/api/placeholder/300/300",
        description: "Cyberpunk-themed knight with glowing circuitry patterns.",
      },
    ],
    "chess-boards": [
      {
        id: "nft-005",
        name: "Starknet Arena",
        rarity: "Legendary",
        owner: "BlockchainDev",
        price: "3.2 ETH",
        image: "/api/placeholder/300/300",
        description:
          "Interactive chessboard with real-time Starknet transaction visualization.",
      },
      {
        id: "nft-006",
        name: "Quantum Field",
        rarity: "Epic",
        owner: "CryptoCollector",
        price: "2.1 ETH",
        image: "/api/placeholder/300/300",
        description:
          "Board with quantum uncertainty effects that influence gameplay.",
      },
      {
        id: "nft-007",
        name: "Neural Network",
        rarity: "Rare",
        owner: "AIGrandmaster",
        price: "1.5 ETH",
        image: "/api/placeholder/300/300",
        description:
          "Board that analyzes and displays potential move patterns.",
      },
      {
        id: "nft-008",
        name: "Galaxy Arena",
        rarity: "Rare",
        owner: "CosmicPlayer",
        price: "1.3 ETH",
        image: "/api/placeholder/300/300",
        description: "Space-themed board with animated celestial effects.",
      },
    ],
    rewards: [
      {
        id: "nft-009",
        name: "Tournament Champion Trophy",
        rarity: "Unique",
        owner: "WorldChampion",
        price: "Not for sale",
        image: "/api/placeholder/300/300",
        description:
          "Exclusive trophy awarded to the winner of the World Blockchain Chess Championship.",
      },
      {
        id: "nft-010",
        name: "Grandmaster Title Badge",
        rarity: "Epic",
        owner: "StarknetMaster",
        price: "Not for sale",
        image: "/api/placeholder/300/300",
        description:
          "Certified proof of achieving Grandmaster status on the platform.",
      },
      {
        id: "nft-011",
        name: "First Mover Medal",
        rarity: "Rare",
        owner: "EarlyAdopter",
        price: "1.8 ETH",
        image: "/api/placeholder/300/300",
        description: "Commemorative medal for early platform participants.",
      },
      {
        id: "nft-012",
        name: "Strategy Genius Badge",
        rarity: "Rare",
        owner: "TacticalPlayer",
        price: "1.1 ETH",
        image: "/api/placeholder/300/300",
        description:
          "Awarded for exceptional strategic gameplay and decision making.",
      },
    ],
  };

  const [selectedNFT, setSelectedNFT] = useState(null);
  const [activeIndex, setActiveIndex] = useState(0);

  const activeNFTs = nftCollections[activeCategory];

  const handlePrev = () => {
    setActiveIndex((prevIndex) =>
      prevIndex === 0 ? activeNFTs.length - 1 : prevIndex - 1
    );
  };

  const handleNext = () => {
    setActiveIndex((prevIndex) =>
      prevIndex === activeNFTs.length - 1 ? 0 : prevIndex + 1
    );
  };

  const getRarityColor = (rarity) => {
    switch (rarity) {
      case "Legendary":
        return "text-amber-400";
      case "Epic":
        return "text-purple-400";
      case "Rare":
        return "text-blue-400";
      case "Unique":
        return "text-rose-400";
      default:
        return "text-gray-400";
    }
  };

  const getRarityBg = (rarity) => {
    switch (rarity) {
      case "Legendary":
        return "bg-amber-500 bg-opacity-20";
      case "Epic":
        return "bg-purple-500 bg-opacity-20";
      case "Rare":
        return "bg-blue-500 bg-opacity-20";
      case "Unique":
        return "bg-rose-500 bg-opacity-20";
      default:
        return "bg-gray-500 bg-opacity-20";
    }
  };

  return (
    <section className="py-16 bg-gradient-to-br from-gray-900 via-black to-gray-900 relative overflow-hidden">
      {/* Animated circuit board background */}
      <div className="absolute inset-0 opacity-10">
        <div className="absolute h-full w-full bg-[url('data:image/svg+xml;base64,PHN2ZyB4bWxucz0iaHR0cDovL3d3dy53My5vcmcvMjAwMC9zdmciIHdpZHRoPSI0MCIgaGVpZ2h0PSI0MCIgdmlld0JveD0iMCAwIDQwIDQwIj48ZyBmaWxsPSJub25lIiBzdHJva2U9IiNGRkYiIHN0cm9rZS13aWR0aD0iMC41IiBzdHJva2Utb3BhY2l0eT0iMC41Ij48cGF0aCBkPSJNMjAgMEwwIDIwIDIwIDQwIDQwIDIweiIvPjxwYXRoIGQ9Ik0wIDAgMjAgMjAgNDAgMCIvPjxwYXRoIGQ9Ik0wIDQwIDIwIDIwIDQwIDQwIi8+PHBhdGggZD0iTTIwIDIwTDAgMCA0MCAwIi8+PHBhdGggZD0iTTIwIDIwTDAgNDAgNDAgNDAiLz48cGF0aCBkPSJNMjAgMjBMMCAwIDAgNDAiLz48cGF0aCBkPSJNMjAgMjBMNDAgMCA0MCA0MCIvPjwvZz48L3N2Zz4=')]"></div>
      </div>

      {/* Glowing orbs */}
      <div className="absolute inset-0 overflow-hidden">
        <div className="absolute w-64 h-64 rounded-full bg-purple-600 blur-3xl opacity-10 -top-20 -left-20"></div>
        <div className="absolute w-96 h-96 rounded-full bg-blue-600 blur-3xl opacity-10 -bottom-40 -right-20"></div>
      </div>

      <div className="container mx-auto px-4 relative z-10">
        <div className="text-center mb-12">
          <h2 className="text-4xl md:text-5xl font-bold mb-4 text-transparent bg-clip-text bg-gradient-to-r from-indigo-400 via-purple-400 to-blue-400">
            NFT Gallery
          </h2>
          <div className="h-1 w-24 bg-gradient-to-r from-indigo-500 to-purple-500 mx-auto mb-6"></div>
          <p className="text-gray-300 max-w-2xl mx-auto text-lg">
            Explore exclusive blockchain-verified chess collectibles and
            tournament rewards
          </p>
        </div>

        {/* Category tabs */}
        <div className="flex flex-wrap justify-center mb-10 gap-2">
          {Object.keys(nftCollections).map((category) => (
            <button
              key={category}
              onClick={() => {
                setActiveCategory(category);
                setActiveIndex(0);
              }}
              className={`px-5 py-2 rounded-full text-sm font-medium transition-all duration-300 ${
                activeCategory === category
                  ? "bg-gradient-to-r from-indigo-600 to-purple-600 text-white shadow-lg shadow-indigo-500/20"
                  : "bg-gray-800 text-gray-400 hover:bg-gray-700"
              }`}
            >
              {category === "chess-pieces"
                ? "Chess Pieces"
                : category === "chess-boards"
                ? "Chess Boards"
                : "Notable Rewards"}
            </button>
          ))}
        </div>

        {/* Featured NFT display */}
        <div className="grid grid-cols-1 lg:grid-cols-5 gap-8 mb-12">
          {/* Main featured NFT */}
          <div className="lg:col-span-3 bg-gray-800 bg-opacity-50 backdrop-blur-lg rounded-xl border border-gray-700 overflow-hidden relative group">
            <div className="absolute inset-0 bg-gradient-to-br from-indigo-600 to-purple-600 opacity-0 group-hover:opacity-20 transition-opacity duration-500"></div>

            <div className="flex flex-col md:flex-row h-full">
              <div className="md:w-3/5 relative">
                <img
                  src={activeNFTs[activeIndex].image}
                  alt={activeNFTs[activeIndex].name}
                  className="w-full h-64 md:h-full object-cover object-center"
                />
                <div
                  className="absolute top-4 left-4 px-3 py-1 rounded-full text-xs font-semibold flex items-center gap-1 capitalize"
                  style={{
                    backgroundColor: "rgba(0,0,0,0.6)",
                    backdropFilter: "blur(10px)",
                  }}
                >
                  <Diamond className="w-3 h-3" />
                  <span
                    className={getRarityColor(activeNFTs[activeIndex].rarity)}
                  >
                    {activeNFTs[activeIndex].rarity}
                  </span>
                </div>
                <button className="absolute bottom-4 right-4 p-2 rounded-full bg-black bg-opacity-50 backdrop-blur-sm text-white hover:bg-opacity-70 transition-all duration-300">
                  <Expand className="w-5 h-5" />
                </button>
              </div>

              <div className="md:w-2/5 p-6 flex flex-col justify-between">
                <div>
                  <h3 className="text-2xl font-bold text-white mb-2">
                    {activeNFTs[activeIndex].name}
                  </h3>
                  <p className="text-gray-400 text-sm mb-4">
                    {activeNFTs[activeIndex].description}
                  </p>

                  <div className="space-y-3">
                    <div className="flex items-center justify-between">
                      <span className="text-gray-500 text-sm">Owner</span>
                      <span className="text-gray-300 font-medium">
                        {activeNFTs[activeIndex].owner}
                      </span>
                    </div>
                    <div className="flex items-center justify-between">
                      <span className="text-gray-500 text-sm">Price</span>
                      <span className="text-indigo-400 font-medium">
                        {activeNFTs[activeIndex].price}
                      </span>
                    </div>
                  </div>
                </div>

                <div className="mt-6 flex justify-between items-center">
                  <button
                    className={`px-4 py-2 rounded-full text-xs font-medium ${
                      activeNFTs[activeIndex].price === "Not for sale"
                        ? "bg-gray-700 text-gray-400 cursor-not-allowed"
                        : "bg-gradient-to-r from-indigo-600 to-purple-600 text-white hover:shadow-lg hover:shadow-indigo-500/30 transition-all duration-300"
                    }`}
                  >
                    {activeNFTs[activeIndex].price === "Not for sale"
                      ? "Not For Sale"
                      : "View Details"}
                  </button>

                  <div className="flex space-x-2">
                    <button
                      onClick={handlePrev}
                      className="p-2 rounded-full bg-gray-700 text-gray-300 hover:bg-gray-600 transition-colors duration-300"
                    >
                      <ChevronLeft className="w-5 h-5" />
                    </button>
                    <button
                      onClick={handleNext}
                      className="p-2 rounded-full bg-gray-700 text-gray-300 hover:bg-gray-600 transition-colors duration-300"
                    >
                      <ChevronRight className="w-5 h-5" />
                    </button>
                  </div>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>
    </section>
  );
};

export default NFTGalleryPreview;

// "use client";
// import React, { useState } from "react";
// import {
//   ChevronLeft,
//   ChevronRight,
//   Diamond,
//   Award,
//   Sparkles,
//   Plus,
//   Expand,
//   Info,
// } from "lucide-react";

// const NFTGalleryPreview = () => {
//   const [activeCategory, setActiveCategory] = useState("chess-pieces");
//   const [activeIndex, setActiveIndex] = useState(0);

//   const nftCollections = {
//     "chess-pieces": [
//       {
//         id: "nft-001",
//         name: "Quantum King",
//         rarity: "Legendary",
//         owner: "CryptoChessmaster",
//         price: "2.5 ETH",
//         image: "/nfts/quantum-king.jpg",
//         description:
//           "Limited edition king piece with quantum effects and special in-game abilities.",
//       },
//       {
//         id: "nft-002",
//         name: "Neural Queen",
//         rarity: "Epic",
//         owner: "StarknetWhiz",
//         price: "1.8 ETH",
//         image: "/nfts/neural-queen.jpg",
//         description:
//           "AI-enhanced queen piece that adapts to your playing style.",
//       },
//     ],
//     "chess-boards": [
//       {
//         id: "nft-005",
//         name: "Starknet Arena",
//         rarity: "Legendary",
//         owner: "BlockchainDev",
//         price: "3.2 ETH",
//         image: "/nfts/starknet-arena.jpg",
//         description:
//           "Interactive chessboard with real-time Starknet transaction visualization.",
//       },
//     ],
//     rewards: [
//       {
//         id: "nft-009",
//         name: "Tournament Champion Trophy",
//         rarity: "Unique",
//         owner: "WorldChampion",
//         price: "Not for sale",
//         image: "/nfts/tournament-trophy.jpg",
//         description:
//           "Exclusive trophy awarded to the winner of the World Blockchain Chess Championship.",
//       },
//     ],
//   };

//   const activeNFTs = nftCollections[activeCategory];

//   const handlePrev = () => {
//     setActiveIndex((prevIndex) =>
//       prevIndex === 0 ? activeNFTs.length - 1 : prevIndex - 1
//     );
//   };

//   const handleNext = () => {
//     setActiveIndex((prevIndex) =>
//       prevIndex === activeNFTs.length - 1 ? 0 : prevIndex + 1
//     );
//   };

//   return (
//     <section className="p-10 bg-black text-white">
//       <h2 className="text-3xl font-bold text-center mb-6">NFT Gallery</h2>
//       <div className="flex justify-center gap-4 mb-6">
//         {Object.keys(nftCollections).map((category) => (
//           <button
//             key={category}
//             onClick={() => {
//               setActiveCategory(category);
//               setActiveIndex(0);
//             }}
//             className={`px-4 py-2 rounded-md ${
//               activeCategory === category ? "bg-blue-500" : "bg-gray-700"
//             }`}
//           >
//             {category.replace("-", " ").toUpperCase()}
//           </button>
//         ))}
//       </div>

//       <div className="relative max-w-lg mx-auto border border-gray-700 rounded-lg overflow-hidden">
//         <img
//           src={activeNFTs[activeIndex].image}
//           alt={activeNFTs[activeIndex].name}
//           className="w-full"
//         />
//         <div className="p-4 bg-gray-800">
//           <h3 className="text-xl font-bold">{activeNFTs[activeIndex].name}</h3>
//           <p className="text-sm text-gray-400">
//             {activeNFTs[activeIndex].description}
//           </p>
//           <p className="mt-2 text-blue-400">{activeNFTs[activeIndex].price}</p>
//         </div>
//         <button
//           onClick={handlePrev}
//           className="absolute left-2 top-1/2 transform -translate-y-1/2 bg-gray-700 p-2 rounded-full"
//         >
//           <ChevronLeft size={20} />
//         </button>
//         <button
//           onClick={handleNext}
//           className="absolute right-2 top-1/2 transform -translate-y-1/2 bg-gray-700 p-2 rounded-full"
//         >
//           <ChevronRight size={20} />
//         </button>
//       </div>
//     </section>
//   );
// };

// export default NFTGalleryPreview;
