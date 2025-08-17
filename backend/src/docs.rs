use std::sync::Arc;

use aide::{
    axum::{
        ApiRouter, IntoApiResponse,
        routing::{get, get_with},
    },
    openapi::{OpenApi, Tag},
    scalar::Scalar,
    transform::TransformOpenApi,
};
use axum::{Extension, Json, response::IntoResponse};
use uuid::Uuid;

pub fn docs_routes() -> ApiRouter {
    // We infer the return types for these routes
    // as an example.
    //
    // As a result, the `serve_redoc` route will
    // have the `text/html` content-type correctly set
    // with a 200 status.
    aide::generate::infer_responses(true);

    let router: ApiRouter = ApiRouter::new()
        .api_route_with(
            "/",
            get_with(
                Scalar::new("/docs/private/api.json")
                    .with_title("Aide Axum")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ),
            |p| p.security_requirement("ApiKey"),
        )
        .route("/private/api.json", get(serve_docs));

    // Afterwards we disable response inference because
    // it might be incorrect for other routes.
    aide::generate::infer_responses(false);

    router
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}

pub fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Jura Hockey API")
        .summary("Hockey games and players catalogization API")
        .description("Hockey games and players catalogization API")
        .tag(Tag {
            name: "event".into(),
            description: Some("Event Management".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "country".into(),
            description: Some("Country Management".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "team".into(),
            description: Some("Team Management".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "team_participation".into(),
            description: Some("Team Participation Management".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "player".into(),
            description: Some("Player Management".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "season".into(),
            description: Some("Season Management".into()),
            ..Default::default()
        })
        .tag(Tag {
            name: "player_contract".into(),
            description: Some("Player Contract Management".into()),
            ..Default::default()
        })
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
        .default_response_with::<Json<crate::errors::ErrorResponse>, _>(|res| {
            res.example(crate::errors::ErrorResponse {
                error: "some error happened".to_string(),
                error_details: None,
                error_id: Uuid::nil(),
            })
        })
}
