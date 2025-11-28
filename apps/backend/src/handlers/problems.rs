use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use serde::Serialize;

use crate::AppState;
use crate::auth::AuthenticatedUser;
use crate::state::{ScanProblem, ScanProblemType};

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProblemResponse {
    pub path: String,
    pub problem_type: String,
    pub message: String,
}

impl From<&ScanProblem> for ProblemResponse {
    fn from(problem: &ScanProblem) -> Self {
        ProblemResponse {
            path: problem.path.to_string_lossy().to_string(),
            problem_type: match &problem.problem_type {
                ScanProblemType::MissingIndexYaml => "MissingIndexYaml",
                ScanProblemType::MissingAudioFile => "MissingAudioFile",
                ScanProblemType::InvalidYamlFormat => "InvalidYamlFormat",
                ScanProblemType::UnableToExtractDuration => "UnableToExtractDuration",
                ScanProblemType::MissingCover => "MissingCover",
                ScanProblemType::InvalidDataFormat => "InvalidDataFormat",
                ScanProblemType::MissingStorageDirectory => "MissingStorageDirectory",
                ScanProblemType::FailedToReadFile => "FailedToReadFile",
                ScanProblemType::FailedToReadDirectory => "FailedToReadDirectory",
                ScanProblemType::FailedToReadDirectoryEntry => "FailedToReadDirectoryEntry",
                ScanProblemType::ScanFailed => "ScanFailed",
                ScanProblemType::RescanFailed => "RescanFailed",
            }
            .to_string(),
            message: problem.message.clone(),
        }
    }
}

pub async fn get_problems(
    State(state): State<AppState>,
    AuthenticatedUser(_user): AuthenticatedUser,
) -> impl IntoResponse {
    let problems: Vec<ProblemResponse> = {
        let problems_guard = state.scan_problems.read().unwrap();
        problems_guard.iter().map(ProblemResponse::from).collect()
    };

    (StatusCode::OK, Json(problems)).into_response()
}
