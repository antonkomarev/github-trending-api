use crate::parser::repository_parser;

#[derive(serde::Deserialize)]
pub struct RequestBody {
    params: Params,
}
#[derive(serde::Deserialize)]
struct Params {
    language: Option<String>,
}

#[derive(serde::Serialize)]
struct Response {
    error: Option<ResponseError>,
    result: Option<ResponseResult>,
}
#[derive(serde::Serialize)]
struct ResponseError {
    code: String,
    detail: String,
}
#[derive(serde::Serialize)]
struct ResponseResult {
    code: String,
    list: Vec<Repository>,
}
#[derive(serde::Serialize)]
struct Repository {
    full_name: String,
    rank: usize,
}

pub async fn invoke(
    axum::Json(payload): axum::Json<RequestBody>,
) -> impl axum::response::IntoResponse {
    match repository_parser::parse(payload.params.language).await {
        Ok(parsed_repository_list) => {
            let repository_list: Vec<Repository> = parsed_repository_list
                .into_iter()
                .map(|repository: repository_parser::Repository| Repository {
                    full_name: repository.full_name,
                    rank: repository.rank,
                })
                .collect();

            let response = Response {
                error: None,
                result: Some(ResponseResult {
                    code: "SUCCESS".to_string(),
                    list: repository_list,
                }),
            };

            (http::StatusCode::OK, axum::Json(response))
        }
        Err(error) => {
            println!("Failed to retrieve repositories: {}", error);

            (
                http::StatusCode::OK,
                axum::Json(Response {
                    error: Some(ResponseError {
                        code: "CANNOT_RETRIEVE_REPOSITORIES".to_string(),
                        detail: format!("{}", error),
                    }),
                    result: None,
                }),
            )
        },
    }
}
