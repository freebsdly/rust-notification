use axum::http::{Method, StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::{BoxError, Json};
use getset::Getters;
use serde::{Deserialize, Serialize};
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder};
use utoipa::{Modify, OpenApi, ToSchema};

/**
 * 响应体
 */
#[allow(unused)]
#[derive(Debug, Deserialize, Clone, Getters, Serialize, ToSchema)]
#[get = "pub"]
pub struct ApiBody<T> {
    #[schema(example = "0")]
    code: i32,
    #[schema(example = "success")]
    message: String,
    data: Option<T>,
}

impl<T> ApiBody<T> {
    pub fn new(code: i32, message: String, data: Option<T>) -> Self {
        ApiBody {
            code,
            message,
            data,
        }
    }

    pub fn success(data: Option<T>) -> Self {
        ApiBody {
            code: 0,
            message: "success".to_string(),
            data,
        }
    }

    pub fn failure(code: i32, message: String) -> Self {
        ApiBody {
            code,
            message,
            data: None,
        }
    }
}

/**
 * 响应转换
 */
impl<T: Serialize> IntoResponse for ApiBody<T> {
    fn into_response(self) -> Response {
        // 可根据 code 映射为 HTTP 状态码
        let status = if self.code == 0 {
            StatusCode::OK
        } else {
            StatusCode::INTERNAL_SERVER_ERROR
        };
        (status, Json(self)).into_response()
    }
}

/**
 * 404 处理
 */
pub async fn handler_404(method: Method, uri: Uri) -> (StatusCode, ApiBody<String>) {
    (
        StatusCode::NOT_FOUND,
        ApiBody::failure(-1, format!("{} {} Not Found", method, uri)),
    )
}

/**
 * 错误处理
 */
pub async fn handle_error(
    // `Method` and `Uri` are extractors so they can be used here
    method: Method,
    uri: Uri,
    // the last argument must be the error itself
    err: BoxError,
) -> (StatusCode, ApiBody<String>) {
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        ApiBody::new(
            -1,
            format!("{} {} failed", method, uri),
            Some(err.to_string()),
        ),
    )
}

// 定义OpenAPI文档
#[derive(OpenApi)]
#[openapi(
    info(
        title = "Web API",
        version = "v1.0.0"
    ),
    modifiers(&SecurityAddon),
    paths(
        index,
        get_users
    ),
)]
pub struct ApiDoc;

// 可选：添加安全方案（如 Bearer Token）
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        use utoipa::openapi::security::SecurityScheme;

        let components = openapi.components.as_mut().unwrap();
        components.add_security_scheme(
            "bearer_auth",
            SecurityScheme::Http(
                HttpBuilder::new()
                    .scheme(HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}

/**
 * 接口
 */
#[utoipa::path(
    get,
    tag = "demos",
    description = "获取demo",
    path = "/api",
    responses(
        (status = 200, description = "Pet found successfully", body = ApiBody<String>),
        (status = NOT_FOUND, description = "Pet was not found")
    )
)]
pub async fn index() -> ApiBody<String> {
    ApiBody::success(Some("Hello World".to_string()))
}

/**
 * 接口
 */
#[utoipa::path(
    get,
    tag = "users",
    description = "用户管理接口",
    path = "/api/users",
    responses(
        (status = 200, description = "Users found successfully"),
        (status = NOT_FOUND, description = "Pet was not found")
    )
)]
pub async fn get_users() -> ApiBody<String> {
    ApiBody::success(Some("Hello World".to_string()))
}
