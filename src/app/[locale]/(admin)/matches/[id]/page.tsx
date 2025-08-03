import MatchDetailsPage from "@/ui/pages/match-details-page";

interface MatchDetailsProps {
  params: { id: string }
}

export default function MatchDetails({ params }: MatchDetailsProps) {
  return <MatchDetailsPage matchId={params.id} />;
}