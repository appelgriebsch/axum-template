use crate::TemplateEngine;

use super::Engine;

use axum::{http::StatusCode, response::IntoResponse};
use tera::{Context, Tera};
use thiserror::Error;

impl TemplateEngine for Engine<Tera> {
    type Error = TeraError;

    fn render<D: serde::Serialize>(&self, key: &str, data: D) -> Result<String, Self::Error> {
        let data = Context::from_serialize(data)?;
        let rendered = self.engine.render(key, &data)?;

        Ok(rendered)
    }
}

#[derive(Error, Debug)]
pub enum TeraError {
    #[error(transparent)]
    RenderError(#[from] tera::Error),
}

impl IntoResponse for TeraError {
    fn into_response(self) -> axum::response::Response {
        (StatusCode::INTERNAL_SERVER_ERROR, self).into_response()
    }
}
