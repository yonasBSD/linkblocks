//! todo: better name than "items"
//! maybe LinkDestinations?

use anyhow::Context;
use sqlx::query;
use uuid::Uuid;

use crate::response_error::ResponseResult;

use super::{AppTx, LinkDestination};

pub async fn search(tx: &mut AppTx, term: &str) -> ResponseResult<Vec<LinkDestination>> {
    let jsons = query!(
        r#"
            select to_jsonb(bookmarks.*) as item
            from bookmarks
            where bookmarks.title like '%' || $1 || '%'
            union
            select to_jsonb(notes.*) as item
            from notes
            where notes.title like '%' || $1 || '%'
            limit 10
        "#,
        term
    )
    .fetch_all(&mut **tx)
    .await?;

    let results = jsons
        .into_iter()
        .map(|row| Ok(serde_json::from_value(row.item.into())?))
        .collect::<anyhow::Result<Vec<LinkDestination>>>()?;

    Ok(results)
}

pub async fn by_id(tx: &mut AppTx, id: Uuid) -> ResponseResult<LinkDestination> {
    let json = query!(
        r#"
            select to_jsonb(bookmarks.*) as item
            from bookmarks
            where bookmarks.id = $1
            union
            select to_jsonb(notes.*) as item
            from notes
            where notes.id = $1
        "#,
        id
    )
    .fetch_one(&mut **tx)
    .await?;

    let results =
        serde_json::from_value(json.item.into()).context("Failed to deserialize item from DB")?;

    Ok(results)
}