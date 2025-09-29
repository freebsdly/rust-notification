pub trait PipelineService {
    /**
     * 获取项目下所有流水线
     */
    fn get_project_pipelines(&self, project_id: &str) -> Result<Vec<String>, anyhow::Error> {
        Err(anyhow::anyhow!("Not implemented"))
    }
}

pub struct BKDevOpsService {}

impl BKDevOpsService {
    pub fn new() -> Self {
        BKDevOpsService {}
    }
}

impl PipelineService for BKDevOpsService {}
