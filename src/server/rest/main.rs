use std::{ error::Error, net::SocketAddr };

use axum::{ routing::get, Router, response::IntoResponse, extract::Query, http::StatusCode, Json };
use serde::{ Deserialize, Serialize };
use utoipa::{ OpenApi, ToSchema, IntoParams };
use utoipa_swagger_ui::SwaggerUi;

#[derive(Serialize, Deserialize, IntoParams, ToSchema)]
pub struct LinkQuery {
    study_uid: Option<String>,
    accession_number: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LinkResponse {
    link: Option<String>,
}
#[derive(Serialize, Deserialize, ToSchema)]
pub struct LinkError {
    error: Option<String>,
}
#[derive(Serialize, Deserialize, ToSchema)]
pub enum LinkEnum {
    LinkResponse(LinkResponse),
    LinkError(LinkError),
}

/// # Метод генерирует ссылку на исследование в архиве
///
/// ## Аргументы:
///
/// * `study_uid` - или используйте уникальный идентификатор исследования
/// * `accession_number` - или используйте номер назначения если в вашей системе все номера назначений уникальны
///
/// ## Пример:
///
///```
/// /link?study_uid=1.110.120.1000
/// или
/// /link?accession_number=100500
///```
///
/// ## Внимание!
/// Данный метод эмулирует поведение сервиса в продуктовой среде, не генерирует действительные ссылки на существующие исследования, возвращаемая ссылка ведет на демонстрационный архив исследования.
/// Для получения успешного результата заполните параметр study_uid любым значением.
/// Для получения исключения заполните accession_number.
#[utoipa::path(
    get,
    path = "/link",
    params(LinkQuery),
    responses(
        (status = 200, description = "Returns link", body = LinkResponse),
        (status = 404, description = "Archive not found", body = LinkError)
    )
)]
pub async fn link_study(query: Query<LinkQuery>) -> impl IntoResponse {
    if query.study_uid.is_some() {
        (
            StatusCode::OK,
            Json(
                LinkEnum::LinkResponse(LinkResponse {
                    link: Some(
                        "https://storage.yandexcloud.net/study-archives/example_archive.zip?X-Amz-Algorithm=AWS4-HMAC-SHA256&X-Amz-Credential=YCAJExKqzrescy78jOniL8lC-%2F20231109%2Fru-central1%2Fs3%2Faws4_request&X-Amz-Date=20231109T080612Z&X-Amz-Expires=2592000&X-Amz-Signature=50497E417A0D420911365C4C52BD53EEB2F7C345AB6F2D38A5D47D04DF13AA6A&X-Amz-SignedHeaders=host".to_string()
                    ),
                })
            ),
        )
    } else {
        (
            StatusCode::NOT_FOUND,
            Json(
                LinkEnum::LinkError(LinkError {
                    error: Some("Not Found".to_string()),
                })
            ),
        )
    }
}

#[derive(OpenApi)]
#[openapi(paths(link_study), components(schemas(LinkQuery, LinkError, LinkResponse)))]
pub struct ApiDoc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));

    let app = Router::new()
        .route("/link", get(link_study))
        .merge(SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi()));

    println!("listening on {}", addr);

    axum::Server::bind(&addr).serve(app.into_make_service()).await.unwrap();

    Ok(())
}
