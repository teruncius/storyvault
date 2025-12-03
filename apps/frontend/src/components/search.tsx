import * as styles from "@sv/fe/components/search.css";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

export function Search() {
    const [search, setSearch] = useState(() => {
        const searchParams = new URLSearchParams(window.location.search);
        return searchParams.get("query") || "";
    });
    const navigate = useNavigate();

    const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        setSearch(e.target.value);
    };

    const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
        e.preventDefault();
        const url = new URL("/library", window.location.origin);
        if (search.length > 0) {
            url.searchParams.set("query", search);
        }
        navigate(url.toString());
    };

    const onClear = () => {
        setSearch("");
        navigate("/library");
    };

    return (
        <form onSubmit={onSubmit} className={styles.searchForm}>
            <div className={styles.searchInputWrapper}>
                <svg
                    className={styles.searchIcon}
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                >
                    <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                    />
                </svg>
                <input
                    type="text"
                    value={search}
                    onChange={onChange}
                    placeholder="Search by title or author..."
                    className={styles.searchInput}
                />
                {search.length > 0 && (
                    <button
                        type="button"
                        onClick={onClear}
                        className={styles.clearButton}
                        aria-label="Clear search"
                    >
                        <svg
                            fill="none"
                            stroke="currentColor"
                            viewBox="0 0 24 24"
                            xmlns="http://www.w3.org/2000/svg"
                            width="16"
                            height="16"
                        >
                            <path
                                strokeLinecap="round"
                                strokeLinejoin="round"
                                strokeWidth={2}
                                d="M6 18L18 6M6 6l12 12"
                            />
                        </svg>
                    </button>
                )}
            </div>
            <button type="submit" className={styles.searchButton}>
                <svg
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                    xmlns="http://www.w3.org/2000/svg"
                    width="20"
                    height="20"
                >
                    <path
                        strokeLinecap="round"
                        strokeLinejoin="round"
                        strokeWidth={2}
                        d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"
                    />
                </svg>
                Search
            </button>
        </form>
    );
}
