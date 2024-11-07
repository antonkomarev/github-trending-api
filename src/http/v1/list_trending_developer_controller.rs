use crate::parser::developer_parser;

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
    list: Vec<Developer>,
}
#[derive(serde::Serialize)]
struct Developer {
    username: String,
    rank: usize,
}

pub async fn invoke(
    axum::Json(payload): axum::Json<RequestBody>,
) -> impl axum::response::IntoResponse {
    match developer_parser::parse(payload.params.language).await {
        Ok(parsed_developer_list) => {
            let developer_list: Vec<Developer> = parsed_developer_list
                .into_iter()
                .map(|developer: developer_parser::Developer| Developer {
                    username: developer.username,
                    rank: developer.rank,
                })
                .collect();

            let response = Response {
                error: None,
                result: Some(ResponseResult {
                    code: "SUCCESS".to_string(),
                    list: developer_list,
                }),
            };

            (http::StatusCode::OK, axum::Json(response))
        }
        Err(error) => (
            http::StatusCode::OK,
            axum::Json(Response {
                error: Some(ResponseError {
                    code: "CANNOT_RETRIEVE_DEVELOPERS".to_string(),
                    detail: format!("{}", error),
                }),
                result: None,
            }),
        ),
    }
}
