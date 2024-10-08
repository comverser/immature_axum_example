use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
};
use sea_orm::{DatabaseConnection, EntityTrait, IntoActiveModel, Set};
use serde::Deserialize;

use crate::periphery_adapters::driven_infrastructure_interface::persistence_adapter::sea_orm::prelude::Tasks;

#[derive(Deserialize)]
pub struct QueryParams {
    soft: bool,
}

pub async fn delete_task(
    Path(task_id): Path<i32>,
    State(database_connection): State<DatabaseConnection>,
    Query(query_params): Query<QueryParams>,
) -> Result<(), StatusCode> {
    if query_params.soft {
        let mut task = if let Some(task) = Tasks::find_by_id(task_id)
            .one(&database_connection)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        {
            task.into_active_model()
        } else {
            return Err(StatusCode::NOT_FOUND);
        };

        let now = chrono::Utc::now();

        task.deleted_at = Set(Some(now.into()));
        Tasks::update(task)
            .exec(&database_connection)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    } else {
        Tasks::delete_by_id(task_id)
            .exec(&database_connection)
            .await
            .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Tasks::delete_many()
        //     .filter(tasks::Column::Id.eq(task_id))
        //     .exec(&database_connection)
        //     .await
        //     .map_err(|_error| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(())
}
