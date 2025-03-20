"use client";

import React, { useState } from "react";

interface NFT {
  id: string;
  name: string;
  rarity: "Legendary" | "Epic" | "Rare" | "Unique";
  owner: string;
  price: string;
  image: string;
  description: string;
}

type NFTCollection = Record<string, NFT[]>;

const NFTGalleryPreview: React.FC = () => {
  const [activeCategory, setActiveCategory] = useState<string>("chess-pieces");

  const nftCollections: NFTCollection = {
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
    ],
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
