import { useState } from "react";
import { ProblemList } from "@sv/fe/components/problem-list";
import { useProblems } from "@sv/fe/hooks/problems";
import * as styles from "./problem-page.css";
import { ProblemTypes } from "@sv/fe/components/problem-types";
import { ProblemSources } from "@sv/fe/components/problem-sources";

export function ProblemPage() {
    const { data: problems } = useProblems();
    const [selectedTypes, setSelectedTypes] = useState<string[]>([]);
    const [selectedSources, setSelectedSources] = useState<string[]>([]);

    const toggleType = (type: string) => {
        setSelectedTypes((prev) =>
            prev.includes(type)
                ? prev.filter((t) => t !== type)
                : [...prev, type],
        );
    };

    const toggleSource = (source: string) => {
        setSelectedSources((prev) =>
            prev.includes(source)
                ? prev.filter((s) => s !== source)
                : [...prev, source],
        );
    };

    const filteredProblems = (problems || []).filter((problem) => {
        const typeMatch =
            selectedTypes.length === 0 ||
            (problem.problemType &&
                selectedTypes.includes(problem.problemType));
        const sourceMatch =
            selectedSources.length === 0 ||
            (problem.source && selectedSources.includes(problem.source));
        return typeMatch && sourceMatch;
    });

    return (
        <div className={styles.twocol}>
            <div className={styles.sidebar}>
                <ProblemTypes
                    problems={problems || []}
                    selectedTypes={selectedTypes}
                    onToggleType={toggleType}
                />
                <ProblemSources
                    problems={problems || []}
                    selectedSources={selectedSources}
                    onToggleSource={toggleSource}
                />
            </div>
            <div className={styles.content}>
                <ProblemList problems={filteredProblems} />
            </div>
        </div>
    );
}
