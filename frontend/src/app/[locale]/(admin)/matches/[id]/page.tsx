import MatchDetailsPage from "@/ui/pages/match-details-page";

interface MatchDetailsProps {
  params: Promise<{ id: string }>
}

export default async function MatchDetails({ params }: MatchDetailsProps) {
  const { id } = await params;
  return <MatchDetailsPage matchId={id} />;
}