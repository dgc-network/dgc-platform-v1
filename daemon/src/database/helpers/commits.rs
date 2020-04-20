// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use super::models::{Commit, NewCommit};
use super::schema::{chain_record, commit};
use super::MAX_COMMIT_NUM;

use diesel::{
    dsl::{delete, insert_into, max, update},
    pg::PgConnection,
    prelude::*,
    result::Error::NotFound,
    QueryResult,
};

pub fn insert_commit(conn: &PgConnection, commit: &NewCommit) -> QueryResult<()> {
    insert_into(commit::table)
        .values(commit)
        .execute(conn)
        .map(|_| ())
}

pub fn resolve_fork(conn: &PgConnection, commit_num: i64) -> QueryResult<()> {
    delete(chain_record::table)
        .filter(chain_record::start_commit_num.ge(commit_num))
        .execute(conn)?;

    update(chain_record::table)
        .filter(chain_record::end_commit_num.ge(commit_num))
        .set(chain_record::end_commit_num.eq(MAX_COMMIT_NUM))
        .execute(conn)?;

    delete(commit::table)
        .filter(commit::commit_num.ge(commit_num))
        .execute(conn)?;

    Ok(())
}

pub fn get_commit_by_commit_num(
    conn: &PgConnection,
    commit_num: i64,
) -> QueryResult<Option<Commit>> {
    commit::table
        .select(commit::all_columns)
        .filter(commit::commit_num.eq(&commit_num))
        .first(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn get_current_commit_id(conn: &PgConnection) -> QueryResult<Option<String>> {
    commit::table
        .select(commit::commit_id)
        .order_by(commit::commit_num.desc())
        .limit(1)
        .first(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn get_next_commit_num(conn: &PgConnection) -> QueryResult<i64> {
    commit::table
        .select(max(commit::commit_num))
        .first(conn)
        .map(|option: Option<i64>| match option {
            Some(num) => num + 1,
            None => 0,
        })
}
