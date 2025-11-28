import { ProblemList } from "@sv/fe/components/problem-list";
import { useProblems } from "@sv/fe/hooks/problems";
import * as styles from "./problem-page.css";
import { ProblemTypes } from "@sv/fe/components/problem-types";
import { ProblemSources } from "@sv/fe/components/problem-sources";

export function ProblemPage() {
    const { data: problems } = useProblems();
    return (
        <div className={styles.twocol}>
            <div className={styles.sidebar}>
                <ProblemTypes problems={problems || []} />
                <ProblemSources problems={problems || []} />
            </div>
            <div className={styles.content}>
                <ProblemList problems={problems || []} />
            </div>
        </div>
    );
}
