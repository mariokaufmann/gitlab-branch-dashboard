export interface DashboardData {
  repositories: RepositoryBranchData[];
}

export interface RepositoryBranchData {
  repository_name: string;
  pull_request_target_branches: PullRequestTargetBranch[];
  standalone_branches: StandaloneBranch[];
}

export interface PullRequestTargetBranch {
  branch_name: string;
  pull_requests: PullRequest[];
}

export interface PullRequest {
  branch_name: string;
  user_profile_image: string;
  comment_count: number;
  last_activity_date: string;
  approved: boolean;
  pipeline_status: PipelineStatus;
}

export interface StandaloneBranch {
  branch_name: string;
  pipeline_status: PipelineStatus;
}

export type PipelineStatus = "Running" | "Successful" | "Failed" | "None";