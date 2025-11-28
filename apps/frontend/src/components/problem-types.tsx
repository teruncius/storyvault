import type { Problem } from "@sv/fe/types/problem";
import * as styles from "@sv/fe/components/problem-types.css";

interface Props {
    problems: Problem[];
    selectedTypes: string[];
    onToggleType: (type: string) => void;
}

export function ProblemTypes(props: Props) {
    const typeCounts = props.problems.reduce(
        (acc, problem) => {
            const type = problem.problemType;
            if (type) {
                acc[type] = (acc[type] || 0) + 1;
            }
            return acc;
        },
        {} as Record<string, number>,
    );

    const sortedTypes = Object.entries(typeCounts).sort((a, b) => b[1] - a[1]);

    return (
        <div className={styles.container}>
            <h2 className={styles.title}>Problem Types</h2>
            <ul className={styles.list}>
                {sortedTypes.map(([type, count]) => (
                    <li
                        key={type}
                        className={`${styles.item} ${props.selectedTypes.includes(type) ? styles.selected : ""}`}
                        onClick={() => props.onToggleType(type)}
                    >
                        <span className={styles.typeName}>{type}</span>
                        <span className={styles.count}>{count}</span>
                    </li>
                ))}
            </ul>
        </div>
    );
}
