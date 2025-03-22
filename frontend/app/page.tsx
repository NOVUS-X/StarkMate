import HeroSection from "@/components/HeroSection";
import HowItWorks from "@/components/HowItWorks";
import KeyFeatures from "@/components/KeyFeatures";
import LiveGamesStatistics from "@/components/LiveGamesStatistics";
import Navbar from "@/components/Navbar";
import NFTGalleryPreview from "@/components/NFTGalleryPreview";
import WaitlistSection from "@/components/WaitlistSection";

export default function Home() {
  return (
    <>
      <Navbar />
      <HeroSection />
      <KeyFeatures />
      <HowItWorks />
      <LiveGamesStatistics />
      <NFTGalleryPreview />
      <WaitlistSection />
    </>
  );
}
