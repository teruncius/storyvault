import { ProblemList } from "@sv/fe/components/problem-list";
import { useProblems } from "@sv/fe/hooks/problems";

export function ProblemPage() {
    const { data: problems } = useProblems();
    return <ProblemList problems={problems || []} />;
}
