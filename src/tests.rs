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
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
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
