use aide::{
    axum::{ApiRouter, IntoApiResponse, routing::get_with},
    transform::TransformOperation,
};
use axum::{
    Extension, Json,
    extract::{Path, Query},
    http::StatusCode,
}; // Add Query back here
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::common::paging::{PagedResponse, Paging};
use crate::country::service::{self, CountryFilters};
use crate::http::ApiContext;

use super::Country;

// Use generic PagedResponse for countries
type PagedCountriesResponse = PagedResponse<Country>;

pub fn country_routes() -> ApiRouter {
    ApiRouter::new()
        .api_route("/", get_with(list_countries, list_countries_docs))
        .api_route(
            "/{id}",
            get_with(get_country, get_country_docs)
                .patch_with(update_country_status, update_country_status_docs),
        )
}

#[derive(Deserialize, Serialize, JsonSchema)]
pub struct CountryQueryParams {
    pub name: Option<String>,
    pub iso2_code: Option<String>,
    pub ioc_code: Option<String>,
    pub enabled: Option<bool>,
    pub iihf: Option<bool>,
    pub is_historical: Option<bool>,
    pub page: Option<usize>,
    pub page_size: Option<usize>,
}

async fn list_countries(
    Query(params): Query<CountryQueryParams>, // This uses axum's Query
    Extension(ctx): Extension<ApiContext>,
) -> impl IntoApiResponse {
    let filters = CountryFilters::new(
        params.enabled,
        params.iihf,
        params.is_historical,
        params.name,
        params.iso2_code,
        params.ioc_code,
    );
    let paging = Some(Paging::new(
        params.page.unwrap_or(1),
        params.page_size.unwrap_or(15),
    ));

    match service::get_countries(&ctx.db, &filters, paging.as_ref()).await {
        Ok(result) => {
            let mapped_items: Vec<_> = result
                .items
                .iter()
                .map(|e| Country {
                    id: e.id,
                    name: e.name.clone(),
                    enabled: e.enabled,
                    iihf: e.iihf,
                    is_historical: e.is_historical,
                    iso2_code: e.iso2_code.clone(),
                    ioc_code: e.ioc_code.clone(),
                })
                .collect();

            // Return the full paged result with metadata
            let response = PagedCountriesResponse::from_result(result, mapped_items);
            Ok(Json(response))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string()))),
    }
}

// And simplify the docs function - remove the parameter documentation for now
fn list_countries_docs(op: TransformOperation) -> TransformOperation {
    op.description("List countries with filtering and paging.")
        .tag("country")
        .response_with::<200, Json<PagedCountriesResponse>, _>(|res| {
            res.example(PagedCountriesResponse::new(
                vec![Country {
                    id: 1,
                    name: "Canada".to_string(),
                    enabled: true,
                    iihf: true,
                    is_historical: false,
                    iso2_code: "CA".to_string(),
                    ioc_code: "CAN".to_string(),
                }],
                213,
                1,
                20,
                11,
                true,
                false,
            ))
        })
}

#[derive(Deserialize, JsonSchema)]
struct SelectCountry {
    /// The ID of the Country.
    id: i64,
}

async fn get_country(
    Extension(ctx): Extension<ApiContext>,
    Path(country): Path<SelectCountry>,
) -> impl IntoApiResponse {
    match service::get_country_by_id(&ctx.db, country.id).await {
        Ok(result) => Ok(Json(Country {
            id: result.id,
            name: result.name,
            enabled: result.enabled,
            iihf: result.iihf,
            is_historical: result.is_historical,
            iso2_code: result.iso2_code,
            ioc_code: result.ioc_code,
        })),
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, Json("Not found".into()))),
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string()))),
    }
}

fn get_country_docs(op: TransformOperation) -> TransformOperation {
    op.description("Get a single Country.")
        .tag("country")
        .response_with::<200, Json<Country>, _>(|res| {
            res.example(Country {
                name: "Example Country".to_string(),
                id: 1,
                enabled: true,
                iihf: true,
                is_historical: false,
                iso2_code: "EX".to_string(),
                ioc_code: "EXC".to_string(),
            })
        })
        .response_with::<404, (), _>(|res| res.description("country was not found"))
}

async fn update_country_status(
    Extension(ctx): Extension<ApiContext>,
    Path(country_id): Path<i64>,
    Json(status): Json<bool>,
) -> impl IntoApiResponse {
    match service::update_country_status(&ctx.db, country_id, status).await {
        Ok(_) => Ok((
            StatusCode::OK,
            Json("Country status updated successfully".to_string()),
        )),
        Err(sqlx::Error::RowNotFound) => {
            Err((StatusCode::NOT_FOUND, Json("Country not found".to_string())))
        }
        Err(err) => Err((StatusCode::INTERNAL_SERVER_ERROR, Json(err.to_string()))),
    }
}

fn update_country_status_docs(op: TransformOperation) -> TransformOperation {
    op.description("Update the status of a country.")
        .tag("country")
        .response_with::<200, Json<String>, _>(|res| {
            res.example("Country status updated successfully".to_string())
        })
        .response_with::<404, (), _>(|res| res.description("Country not found"))
        .response_with::<500, (), _>(|res| res.description("Internal server error"))
}
