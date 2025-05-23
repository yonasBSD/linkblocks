use activitypub_federation::fetch::object_id::ObjectId;
use sqlx::{FromRow, query_as};
use time::OffsetDateTime;
use url::Url;
use uuid::Uuid;

use super::AppTx;
use crate::{forms::ap_users::CreateApUser, response_error::ResponseResult};

#[derive(FromRow, Debug)]
pub struct ApUser {
    pub id: Uuid,

    pub ap_id: ObjectId<ApUser>,
    pub username: String,
    pub inbox_url: Url,
    pub public_key: String,
    pub private_key: Option<String>,
    pub last_refreshed_at: OffsetDateTime,
    pub display_name: Option<String>,
    pub bio: Option<String>,
}

#[derive(FromRow, Debug)]
struct ApUserRow {
    pub id: Uuid,

    pub ap_id: String,
    pub username: String,
    pub inbox_url: String,
    pub public_key: String,
    pub private_key: Option<String>,
    pub last_refreshed_at: OffsetDateTime,
    pub display_name: Option<String>,
    pub bio: Option<String>,
}

impl TryFrom<ApUserRow> for ApUser {
    fn try_from(value: ApUserRow) -> anyhow::Result<Self> {
        Ok(ApUser {
            id: value.id,
            ap_id: value.ap_id.parse()?,
            username: value.username,
            inbox_url: value.inbox_url.parse()?,
            public_key: value.public_key,
            private_key: value.private_key,
            last_refreshed_at: value.last_refreshed_at,
            display_name: value.display_name,
            bio: value.bio,
        })
    }

    type Error = anyhow::Error;
}

pub async fn insert(tx: &mut AppTx, create_user: CreateApUser) -> ResponseResult<ApUser> {
    let user = query_as!(
        ApUserRow,
        r#"
        insert into ap_users
        (
            ap_id,
            username,
            inbox_url,
            public_key,
            private_key,
            last_refreshed_at,
            display_name,
            bio
        )
        values ($1, $2, $3, $4, $5, $6, $7, $8)
        returning *
        "#,
        create_user.ap_id.to_string(),
        create_user.username,
        create_user.inbox_url.to_string(),
        create_user.public_key,
        create_user.private_key,
        create_user.last_refreshed_at,
        create_user.display_name,
        create_user.bio,
    )
    .fetch_one(&mut **tx)
    .await?
    .try_into()?;

    Ok(user)
}

pub async fn upsert(tx: &mut AppTx, create_user: CreateApUser) -> ResponseResult<ApUser> {
    let user = query_as!(
        ApUserRow,
        r#"
        insert into ap_users
        (
            ap_id,
            username,
            inbox_url,
            public_key,
            private_key,
            last_refreshed_at,
            display_name,
            bio
        )
        values ($1, $2, $3, $4, $5, $6, $7, $8)
        on conflict(ap_id) do update set
            ap_id = $1,
            username = $2,
            inbox_url = $3,
            public_key = $4,
            private_key = $5,
            last_refreshed_at = $6,
            display_name = $7,
            bio = $8
        returning *
        "#,
        create_user.ap_id.to_string(),
        create_user.username,
        create_user.inbox_url.to_string(),
        create_user.public_key,
        create_user.private_key,
        create_user.last_refreshed_at,
        create_user.display_name,
        create_user.bio,
    )
    .fetch_one(&mut **tx)
    .await?
    .try_into()?;

    Ok(user)
}

pub async fn read_by_ap_id(tx: &mut AppTx, ap_id: &Url) -> ResponseResult<ApUser> {
    let user = query_as!(
        ApUserRow,
        r#"
        select * from ap_users
        where ap_id = $1
        "#,
        ap_id.to_string()
    )
    .fetch_one(&mut **tx)
    .await?
    .try_into()?;

    Ok(user)
}

pub async fn read_by_username(tx: &mut AppTx, username: &str) -> ResponseResult<ApUser> {
    let user = query_as!(
        ApUserRow,
        r#"
        select * from ap_users
        where username = $1
        "#,
        username
    )
    .fetch_one(&mut **tx)
    .await?
    .try_into()?;

    Ok(user)
}
