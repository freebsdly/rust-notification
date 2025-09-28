use crate::conf::DevOpsOptions;
use anyhow::{anyhow, Context};
use getset::Getters;
use serde::Deserialize;

#[allow(unused)]
#[derive(Debug, Deserialize, Clone, Getters)]
#[serde(rename_all = "camelCase", default)]
#[get = "pub"]
pub struct PageRecords<T> {
    count: i64,
    page: u32,
    page_size: u32,
    total_pages: u32,
    records: Vec<T>,
}

impl<T> Default for PageRecords<T> {
    fn default() -> Self {
        Self {
            count: 0,
            page: 0,
            page_size: 0,
            total_pages: 0,
            records: Vec::new(),
        }
    }
}

#[allow(unused)]
#[derive(Debug, Deserialize, Clone, Getters)]
#[serde(rename_all = "camelCase", default)]
pub struct DevOpsApiBody<T> {
    status: String,
    data: Option<T>,
    message: Option<String>,
    code: Option<i32>,
    trace_id: Option<String>,
}

impl<T> Default for DevOpsApiBody<T> {
    fn default() -> Self {
        Self {
            status: Default::default(),
            data: None,
            message: None,
            code: None,
            trace_id: None,
        }
    }
}

#[allow(unused)]
#[derive(Debug, Deserialize, Clone, Getters)]
#[serde(rename_all = "camelCase")]
#[get = "pub"]
pub struct PipelineInfo {
    pub project_id: String,
    pub pipeline_id: String,
    pub pipeline_name: String,
    pub pipeline_desc: String,
    pub task_count: i32,
    pub build_count: i32,
    pub lock: bool,
    pub can_manual_startup: bool,
    pub latest_build_start_time: i64,
    pub latest_build_end_time: i64,
    pub latest_build_num: i32,
    pub latest_build_estimated_execution_seconds: i32,
    pub deployment_time: i64,
    pub create_time: i64,
    pub update_time: i64,
    pub pipeline_version: i32,
    pub current_timestamp: i64,
    pub running_build_count: i32,
    pub has_permission: bool,
    pub has_collect: bool,
    pub latest_build_user_id: String,
    pub instance_from_template: bool,
    pub template_id: String,
    pub version_name: String,
    pub version: i32,
    pub updater: String,
    pub creator: String,
    pub last_build_total_count: i32,
    pub last_build_finish_count: i32,
    pub delete: bool,
}

impl Default for PipelineInfo {
    fn default() -> Self {
        Self {
            project_id: String::default(),
            pipeline_id: String::default(),
            pipeline_name: String::default(),
            pipeline_desc: String::default(),
            task_count: 0,
            build_count: 0,
            lock: false,
            can_manual_startup: false,
            latest_build_start_time: 0,
            latest_build_end_time: 0,
            latest_build_num: 0,
            latest_build_estimated_execution_seconds: 0,
            deployment_time: 0,
            create_time: 0,
            update_time: 0,
            pipeline_version: 0,
            current_timestamp: 0,
            running_build_count: 0,
            has_permission: false,
            has_collect: false,
            latest_build_user_id: String::default(),
            instance_from_template: false,
            template_id: String::default(),
            version_name: String::default(),
            version: 0,
            updater: String::default(),
            creator: String::default(),
            last_build_total_count: 0,
            last_build_finish_count: 0,
            delete: false,
        }
    }
}

#[allow(unused)]
pub struct DevOpsApiClient {
    client: reqwest::Client,
    options: DevOpsOptions,
}

impl DevOpsApiClient {
    pub(crate) fn new(options: DevOpsOptions) -> Self {
        DevOpsApiClient {
            client: reqwest::Client::new(),
            options,
        }
    }

    pub async fn get_project_pipelines(
        &self,
        project_id: String,
    ) -> Result<Vec<PipelineInfo>, anyhow::Error> {
        let url = format!(
            "{}/projects/CCI/api/service/open/pipeline_get",
            self.options.base_url(),
        );
        let response = self
            .client
            .get(url)
            .header("X-DEVOPS-ACCESS-TOKEN", self.options.access_token())
            .header("X-DEVOPS-UID", self.options.user_id())
            .query(&[
                ("projectCode", project_id.as_str()),
                ("page", "1"),
                ("pageSize", "100"),
            ])
            .send()
            .await
            .context("get project pipelines failed")?;

        if response.status().is_success() {
            let body = response
                .json::<DevOpsApiBody<PageRecords<PipelineInfo>>>()
                .await
                .context("convert body to pipeline struct failed")?;

            match body.data {
                None => Ok(vec![]),
                Some(records) => Ok(records.records),
            }
        } else {
            let body = response
                .json::<DevOpsApiBody<String>>()
                .await
                .context("convert body to error struct failed")?;
            // 创建一个错误响应
            Err(anyhow!(format!(
                "get project {} pipelines failed.  {}",
                project_id,
                body.message
                    .unwrap_or_else(|| "message is empty".to_string())
            )))
        }
    }
}
