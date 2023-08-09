/*
 *     Copyright (C) 2023 Fritz Ochsmann
 *
 *     This program is free software: you can redistribute it and/or modify
 *     it under the terms of the GNU Affero General Public License as published
 *     by the Free Software Foundation, either version 3 of the License, or
 *     (at your option) any later version.
 *
 *     This program is distributed in the hope that it will be useful,
 *     but WITHOUT ANY WARRANTY; without even the implied warranty of
 *     MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *     GNU Affero General Public License for more details.
 *
 *     You should have received a copy of the GNU Affero General Public License
 *     along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

use crate::ConnectionInfo;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::engine::remote::ws::Client;
use surrealdb::opt::auth::Scope;
use surrealdb::sql::Thing;
use surrealdb::Surreal;

#[derive(Serialize, Debug)]
struct AccountAuthParams<'a> {
    username: &'a str,
    password: &'a str,
}

#[derive(Serialize, Debug)]
struct VisitorAuthParams<'a> {
    id: &'a str,
}

#[derive(Deserialize, Serialize, Debug)]
struct Account {
    id: Thing,
    username: String,
    password: String,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Event {
    id: Thing,
    title: String,
    organizer: Thing,
    dates: Vec<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

#[derive(Deserialize, Serialize, Debug, PartialEq)]
struct Visitor {
    id: Thing,
    username: String,
    event: Thing,
    visitable: Vec<DateTime<Utc>>,
    created_at: DateTime<Utc>,
}

async fn signup(
    connection: &Surreal<Client>,
    namespace: &str,
    database: &str,
    username: &str,
    password: &str,
) -> anyhow::Result<()> {
    assert!(connection
        .signup(Scope {
            namespace,
            database,
            scope: "account",
            params: AccountAuthParams { username, password },
        })
        .await
        .is_ok());

    Ok(())
}

async fn start_default(info: ConnectionInfo) -> anyhow::Result<ConnectionInfo> {
    signup(
        info.connection(),
        info.namespace(),
        info.database(),
        "username",
        "password",
    )
    .await?;

    assert!(info
        .connection
        .signin(Scope {
            namespace: info.namespace(),
            database: info.database(),
            scope: "account",
            params: AccountAuthParams {
                username: "username",
                password: "password",
            },
        })
        .await
        .is_ok());

    Ok(info)
}

#[tokio::test]
async fn test_signup() -> anyhow::Result<()> {
    let info = crate::connect().await?;
    start_default(info).await?;

    Ok(())
}

#[tokio::test]
async fn test_select_account() -> anyhow::Result<()> {
    let info = crate::connect().await?;
    signup(
        info.connection(),
        info.namespace(),
        info.database(),
        "first",
        "password",
    )
    .await?;
    let info = start_default(info).await?;

    let response = info.connection().select::<Vec<Account>>("account").await?;
    assert_eq!(1, response.len());
    assert_eq!("username", response.first().unwrap().username);

    Ok(())
}

async fn create_event(connection: &Surreal<Client>) -> anyhow::Result<Event> {
    let dates = vec![Utc::now()];

    let event = connection
        .create("event")
        .content(json! ({
            "title": "Event",
            "dates": dates
        }))
        .await?;

    Ok(event)
}

#[tokio::test]
async fn test_create_event() -> anyhow::Result<()> {
    let info = crate::connect().await?;
    let info = start_default(info).await?;
    let connection = info.connection();

    // create the event
    assert!(create_event(connection).await.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_delete_event() -> anyhow::Result<()> {
    let info = crate::connect().await?;
    let info = start_default(info).await?;
    let connection = info.connection();
    let event = create_event(connection).await?;

    let _ = connection.delete::<Option<Event>>(event.id).await?;
    assert_eq!(0, connection.select::<Vec<Event>>("event").await?.len());

    Ok(())
}

#[tokio::test]
async fn test_update_event() -> anyhow::Result<()> {
    let info = crate::connect().await?;
    let info = start_default(info).await?;
    let connection = info.connection();
    let event = create_event(connection).await?;

    let dates = vec![Utc::now() + Duration::days(1)];
    connection
        .query("UPDATE $event SET dates=$dates")
        .bind(("event", event.id.clone()))
        .bind(("dates", dates.clone()))
        .await?;
    let event: Event = connection.select::<Option<Event>>(event.id.clone()).await?;
    assert_eq!(dates, event.dates);

    Ok(())
}

#[tokio::test]
async fn test_get_events() -> anyhow::Result<()> {
    let info = crate::connect().await?;
    let info = start_default(info).await?;
    let connection = info.connection();
    create_event(connection).await?;

    assert_eq!(1, connection.select::<Vec<Event>>("event").await?.len());
    Ok(())
}

async fn create_visitor(
    username: &str,
    event: Thing,
    connection: &Surreal<Client>,
) -> anyhow::Result<Visitor> {
    let visitor = connection
        .query("CREATE visitor SET username = $username, event = $event")
        .bind(("username", username))
        .bind(("event", event))
        .await?
        .take::<Option<Visitor>>(0)?;

    Ok(visitor.unwrap())
}

#[tokio::test]
async fn test_create_visitor() -> anyhow::Result<()> {
    let info = crate::connect().await?;
    let info = start_default(info).await?;
    let connection = info.connection();
    let event = create_event(connection).await?;

    assert!(create_visitor("username", event.id.clone(), connection)
        .await
        .is_ok());
    Ok(())
}

#[tokio::test]
async fn test_get_visitor() -> anyhow::Result<()> {
    let info = crate::connect().await?;
    let info = start_default(info).await?;
    let connection = info.connection();
    let event = create_event(connection).await?;
    let visitor = create_visitor("username", event.id.clone(), connection).await?;

    assert_eq!(
        visitor,
        connection
            .select::<Option<Visitor>>(visitor.id.clone())
            .await?
    );
    Ok(())
}

#[tokio::test]
async fn test_delete_visitor() -> anyhow::Result<()> {
    let info = crate::connect().await?;
    let info = start_default(info).await?;
    let connection = info.connection();
    let event = create_event(connection).await?;
    let visitor = create_visitor("username", event.id.clone(), connection).await?;

    let _ = connection.delete::<Option<Visitor>>(visitor.id).await?;
    assert_eq!(0, connection.select::<Vec<Visitor>>("visitor").await?.len());

    Ok(())
}
