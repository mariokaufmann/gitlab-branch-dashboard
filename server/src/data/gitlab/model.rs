use serde_derive::Deserialize;

pub struct BranchDetails {
    pub details_response: BranchResponse,
    pub pipeline_response: Option<PipelineResponse>,
    pub job_response: Option<JobResponse>,
}

pub struct MergeRequestDetails {
    pub details_response: SingleMergeRequestResponse,
    pub approvals_response: MergeRequestApprovalsResponse,
    pub job_response: Option<JobResponse>,
}

#[derive(Deserialize)]
pub struct ProjectResponse {
    pub id: u32,
    pub web_url: String,
}

#[derive(Deserialize)]
pub struct BranchResponse {
    pub name: String,
}

#[derive(Deserialize)]
pub struct MergeRequestResponse {
    pub iid: u32,
}

#[derive(Deserialize)]
pub struct SingleMergeRequestResponse {
    pub source_branch: String,
    pub target_branch: String,
    pub author: GitlabUserResponse,
    pub updated_at: String,
    pub user_notes_count: u32,
    pub has_conflicts: bool,
    pub blocking_discussions_resolved: bool,
    pub pipeline: Option<PipelineResponse>,
    pub web_url: String,
}

#[derive(Deserialize)]
pub struct PipelineResponse {
    pub id: u32,
    pub status: GitlabPipelineStatus,
}

#[derive(Deserialize)]
pub struct JobResponse {
    pub web_url: String,
}

#[derive(Deserialize)]
pub struct MergeRequestApprovalsResponse {
    pub approved: bool,
}

#[derive(Deserialize)]
pub struct GitlabUserResponse {
    pub name: String,
    pub avatar_url: String,
}

#[derive(Deserialize)]
pub enum GitlabPipelineStatus {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "running")]
    Running,
    #[serde(rename = "failed")]
    Failed,
    #[serde(rename = "created")]
    Created,
    #[serde(rename = "waiting_for_resource")]
    WaitingForResource,
    #[serde(rename = "preparing")]
    Preparing,
    #[serde(rename = "pending")]
    Pending,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "skipped")]
    Skipped,
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "scheduled")]
    Scheduled,
}
