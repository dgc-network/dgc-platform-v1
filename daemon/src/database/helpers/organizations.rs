// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use super::models::{NewOrganization, Organization};
use super::schema::organization;
use super::MAX_COMMIT_NUM;

use diesel::{
    dsl::{insert_into, update},
    pg::PgConnection,
    prelude::*,
    result::Error::NotFound,
    QueryResult,
};

pub fn insert_organizations(
    conn: &PgConnection,
    organizations: &[NewOrganization],
) -> QueryResult<()> {
    for org in organizations {
        update_org_end_commit_num(conn, &org.org_id, org.start_commit_num)?;
    }

    insert_into(organization::table)
        .values(organizations)
        .execute(conn)
        .map(|_| ())
}

fn update_org_end_commit_num(
    conn: &PgConnection,
    org_id: &str,
    current_commit_num: i64,
) -> QueryResult<()> {
    update(organization::table)
        .filter(
            organization::org_id
                .eq(org_id)
                .and(organization::end_commit_num.eq(MAX_COMMIT_NUM)),
        )
        .set(organization::end_commit_num.eq(current_commit_num))
        .execute(conn)
        .map(|_| ())
}

pub fn list_organizations(
    conn: &PgConnection,
    service_id: Option<&str>,
) -> QueryResult<Vec<Organization>> {
    let mut query = organization::table
        .into_boxed()
        .select(organization::all_columns)
        .filter(organization::end_commit_num.eq(MAX_COMMIT_NUM));

    if let Some(service_id) = service_id {
        query = query.filter(organization::service_id.eq(service_id))
    } else {
        query = query.filter(organization::service_id.is_null())
    }

    query.load::<Organization>(conn)
}

pub fn fetch_organization(
    conn: &PgConnection,
    organization_id: &str,
    service_id: Option<&str>,
) -> QueryResult<Option<Organization>> {
    let mut query = organization::table
        .into_boxed()
        .select(organization::all_columns)
        .filter(
            organization::org_id
                .eq(organization_id)
                .and(organization::end_commit_num.eq(MAX_COMMIT_NUM)),
        );

    if let Some(service_id) = service_id {
        query = query.filter(organization::service_id.eq(service_id));
    } else {
        query = query.filter(organization::service_id.is_null());
    };

    query
        .first(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}
