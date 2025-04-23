import Footer from "../../components/wip-landing/Footer";
import HeroSection from "../../components/wip-landing/HeroSection";
import HowItWorks from "../../components/wip-landing/HowItWorks";
import KeyFeatures from "../../components/wip-landing/KeyFeatures";
import LiveGamesStatistics from "../../components/wip-landing/LiveGamesStatistics";
import Navbar from "../../components/wip-landing/Navbar";
import NFTGalleryPreview from "../../components/wip-landing/NFTGalleryPreview";
import WaitlistSection from "../../components/wip-landing/WaitlistSection";

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
      <Footer />
    </>
  );
}
