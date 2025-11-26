import * as styles from "@sv/fe/components/problem-list.css";
import type { Problem } from "@sv/fe/types/problem";

interface Props {
    problems: Problem[];
}

export function ProblemList(props: Props) {
    if (props.problems.length === 0) {
        return null;
    }

    return (
        <div className={styles.container}>
            <h2 className={styles.title}>Scanning Problems</h2>
            <div className={styles.list}>
                {props.problems.map((problem, index) => (
                    <div key={index} className={styles.item}>
                        <div className={styles.problemType}>
                            {problem.problem_type}
                        </div>
                        <div className={styles.path}>{problem.path}</div>
                        <div className={styles.message}>{problem.message}</div>
                    </div>
                ))}
            </div>
        </div>
    );
}
