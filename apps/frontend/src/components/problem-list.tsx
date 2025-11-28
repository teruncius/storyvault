import * as styles from "@sv/fe/components/problem-list.css";
import type { Problem } from "@sv/fe/types/problem";

interface Props {
    problems: Problem[];
}

export function ProblemList(props: Props) {
    if (props.problems.length === 0) {
        return null;
    }

    // Group problems by path
    const problemsByPath = props.problems.reduce(
        (acc, problem) => {
            if (!acc[problem.path]) {
                acc[problem.path] = [];
            }
            acc[problem.path].push(problem);
            return acc;
        },
        {} as Record<string, Problem[]>,
    );

    return (
        <div className={styles.container}>
            <div className={styles.list}>
                {Object.entries(problemsByPath).map(([path, problems]) => (
                    <div key={path} className={styles.pathGroup}>
                        {problems.map((problem, index) => (
                            <div key={index} className={styles.item}>
                                <div className={styles.problemType}>
                                    {problem.problemType}
                                </div>
                                <div className={styles.path}>{path}</div>
                                <div className={styles.message}>
                                    {problem.message}
                                </div>
                            </div>
                        ))}
                    </div>
                ))}
            </div>
        </div>
    );
}
