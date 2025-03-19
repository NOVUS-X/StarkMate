import HeroSection from "@/components/HeroSection";
import HowItWorks from "@/components/HowItWorks";
import KeyFeatures from "@/components/KeyFeatures";
import LiveGamesStatistics from "@/components/LiveGamesStatitics";
import Navbar from "@/components/Navbar";
import NFTGalleryPreview from "@/components/NFTGalleryPreview";

export default function Home() {
  return (
    <div>
      <Navbar/>
      <HeroSection/>
      <KeyFeatures/>
      <HowItWorks/>
      <LiveGamesStatistics/>
      <NFTGalleryPreview/>
      </div>
  );
}
