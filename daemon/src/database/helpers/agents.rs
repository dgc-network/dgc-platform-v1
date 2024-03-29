// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use super::models::{Agent, NewAgent};
use super::schema::agent;
use super::MAX_COMMIT_NUM;

use diesel::{
    dsl::{insert_into, update},
    pg::PgConnection,
    prelude::*,
    result::Error::NotFound,
    QueryResult,
};

pub fn insert_agents(conn: &PgConnection, agents: &[NewAgent]) -> QueryResult<()> {
    for agent in agents {
        update_agent_end_commit_num(conn, &agent.public_key, agent.start_commit_num)?;
    }

    insert_into(agent::table)
        .values(agents)
        .execute(conn)
        .map(|_| ())
}

fn update_agent_end_commit_num(
    conn: &PgConnection,
    public_key: &str,
    current_commit_num: i64,
) -> QueryResult<()> {
    update(agent::table)
        .filter(
            agent::public_key
                .eq(public_key)
                .and(agent::end_commit_num.eq(MAX_COMMIT_NUM)),
        )
        .set(agent::end_commit_num.eq(current_commit_num))
        .execute(conn)
        .map(|_| ())
}

pub fn get_agents(conn: &PgConnection, service_id: Option<&str>) -> QueryResult<Vec<Agent>> {
    let mut query = agent::table
        .into_boxed()
        .select(agent::all_columns)
        .filter(agent::end_commit_num.eq(MAX_COMMIT_NUM));

    if let Some(service_id) = service_id {
        query = query.filter(agent::service_id.eq(service_id));
    } else {
        query = query.filter(agent::service_id.is_null());
    }

    query.load::<Agent>(conn)
}

pub fn get_agent(
    conn: &PgConnection,
    public_key: &str,
    service_id: Option<&str>,
) -> QueryResult<Option<Agent>> {
    let mut query = agent::table.into_boxed().select(agent::all_columns).filter(
        agent::public_key
            .eq(public_key)
            .and(agent::end_commit_num.eq(MAX_COMMIT_NUM)),
    );

    if let Some(service_id) = service_id {
        query = query.filter(agent::service_id.eq(service_id));
    } else {
        query = query.filter(agent::service_id.is_null());
    }

    query
        .first(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}
