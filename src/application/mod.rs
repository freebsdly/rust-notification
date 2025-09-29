pub trait PipelineService {
    fn get_project_pipelines(&self, project_id: &str) -> Result<Vec<String>, anyhow::Error> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}

pub struct BKDevOpsService {}

impl PipelineService for BKDevOpsService {}
