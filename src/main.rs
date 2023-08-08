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

#[macro_use]
extern crate tracing;

use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::Surreal;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use version_compare::{Cmp, Version};

#[cfg(test)]
use getset::Getters;

#[cfg(test)]
mod tests;

const SURREALDB_USERNAME: &str = "SURREALDB_USERNAME";
const SURREALDB_PASSWORD: &str = "SURREALDB_PASSWORD";
const SURREALDB_ENDPOINT: &str = "SURREALDB_ENDPOINT";

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer())
        .init();

    let _ = connect().await?;

    Ok(())
}

#[allow(unused)]
macro_rules! migrations {
    ($($version:expr), +) => {{
        let mut migrations = Vec::new();

        $(
            migrations.push(($version, include_str!(format!("{}.suql", $version).as_str())));
        ),+

        migrations
    }};
}

#[derive(Debug, Clone)]
#[cfg_attr(test, derive(Getters))]
#[cfg_attr(test, get = "pub")]
pub struct ConnectionInfo {
    namespace: String,
    database: String,
    connection: Surreal<Client>,
}

async fn connect() -> anyhow::Result<ConnectionInfo> {
    // read all env variables
    let username = std::env::var(SURREALDB_USERNAME)
        .unwrap_or_else(|_| panic!("Missing {SURREALDB_USERNAME} env variable"));
    let password = std::env::var(SURREALDB_PASSWORD)
        .unwrap_or_else(|_| panic!("Missing {SURREALDB_PASSWORD} env variable"));
    let endpoint = std::env::var(SURREALDB_ENDPOINT)
        .unwrap_or_else(|_| panic!("Missing {SURREALDB_ENDPOINT} env variable"));

    info!("Trying to connect to surrealdb at {}", endpoint);
    // connect to the database
    let connection: Surreal<Client> = Surreal::new::<Ws>(endpoint).await?;
    info!("Connected to surrealdb");

    info!("Trying to authorize with surrealdb as {}", username);
    connection
        .signin(Root {
            username: username.as_str(),
            password: password.as_str(),
        })
        .await?;
    info!("Authorized with surrealdb");

    // use namespace and database
    #[cfg(not(test))]
    let database = "eventplanner".to_owned();
    #[cfg(not(test))]
    let namespace = "production".to_owned();
    #[cfg(test)]
    let database = nanoid::nanoid!();
    #[cfg(test)]
    let namespace = "test".to_owned();
    connection
        .use_ns(namespace.as_str())
        .use_db(database.as_str())
        .await?;

    info!("Performing migrations");
    migrate(&connection, env!("CARGO_PKG_VERSION"), Vec::new()).await?;

    connection.query(include_str!("./up.suql")).await?;
    info!("Initiated tables");

    Ok(ConnectionInfo {
        database,
        namespace,
        connection,
    })
}

async fn migrate(
    connection: &Surreal<Client>,
    current_version: &'static str,
    migrations: Vec<(&'static str, &'static str)>,
) -> anyhow::Result<()> {
    // take the last as response, which contains the last migrated version
    let last = connection
        .query(
            "DEFINE TABLE migration SCHEMALESS;
            DEFINE FIELD version     on TABLE migration TYPE string ASSERT $value IS NOT NULL;
            DEFINE FIELD created_at  on TABLE migration TYPE datetime VALUE time::now();",
        )
        .query("SELECT version, created_at FROM migration ORDER BY created_at DESC LIMIT 1")
        .await?
        .check()?
        .take::<Option<String>>((1, "version"))?;

    if let Some(last) = last {
        // only proceed if the  last version is not equal to the current version
        if !last.as_str().eq(current_version) {
            // iterate through the given migrations
            for (version, migration) in migrations {
                if Version::from(last.as_str())
                    .unwrap()
                    .compare_to(Version::from(current_version).unwrap(), Cmp::Lt)
                {
                    info!("Executing surrealdb migration to {version}");
                    // execute the migration query and mark it as done
                    connection
                        .query(migration)
                        .query("CREATE migration SET version = $version")
                        .bind(("version", version))
                        .await?
                        .check()?;
                }
            }
        }
    } else {
        // insert the current version as the last version
        connection
            .query("CREATE migration SET version = $version")
            .bind(("version", current_version))
            .await?
            .check()?;
    }

    Ok(())
}
