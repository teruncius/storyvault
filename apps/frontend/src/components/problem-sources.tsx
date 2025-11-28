import type { Problem } from "@sv/fe/types/problem";
import * as styles from "@sv/fe/components/problem-sources.css";

interface Props {
    problems: Problem[];
    selectedSources: string[];
    onToggleSource: (source: string) => void;
}

export function ProblemSources(props: Props) {
    const sourceCounts = props.problems.reduce(
        (acc, problem) => {
            const source = problem.source;
            if (source) {
                acc[source] = (acc[source] || 0) + 1;
            }
            return acc;
        },
        {} as Record<string, number>,
    );

    const sortedSources = Object.entries(sourceCounts).sort(
        (a, b) => b[1] - a[1],
    );

    return (
        <div className={styles.container}>
            <h2 className={styles.title}>Problem Sources</h2>
            <ul className={styles.list}>
                {sortedSources.map(([source, count]) => (
                    <li
                        key={source}
                        className={`${styles.item} ${props.selectedSources.includes(source) ? styles.selected : ""}`}
                        onClick={() => props.onToggleSource(source)}
                    >
                        <span className={styles.sourceName}>{source}</span>
                        <span className={styles.count}>{count}</span>
                    </li>
                ))}
            </ul>
        </div>
    );
}
