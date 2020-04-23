// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use super::models::{PropertyDefinition, Schema, NewPropertyDefinition, NewSchema};
use super::schema::{property_definition, dgc_platform_schema};
use super::MAX_COMMIT_NUM;

use diesel::{
    dsl::{insert_into, update},
    pg::PgConnection,
    prelude::*,
    result::Error::NotFound,
    QueryResult,
};

pub fn insert_grid_schemas(conn: &PgConnection, schemas: &[NewSchema]) -> QueryResult<()> {
    for schema in schemas {
        update_grid_schema_end_commit_num(conn, &schema.name, schema.start_commit_num)?;
    }

    insert_into(dgc_platform_schema::table)
        .values(schemas)
        .execute(conn)
        .map(|_| ())
}

pub fn insert_grid_property_definitions(
    conn: &PgConnection,
    definitions: &[NewPropertyDefinition],
) -> QueryResult<()> {
    for definition in definitions {
        update_definition_end_commit_num(
            conn,
            &definition.name,
            &definition.schema_name,
            definition.start_commit_num,
        )?;
    }

    insert_into(property_definition::table)
        .values(definitions)
        .execute(conn)
        .map(|_| ())
}

pub fn update_grid_schema_end_commit_num(
    conn: &PgConnection,
    name: &str,
    current_commit_num: i64,
) -> QueryResult<()> {
    update(dgc_platform_schema::table)
        .filter(
            dgc_platform_schema::name
                .eq(name)
                .and(dgc_platform_schema::end_commit_num.eq(MAX_COMMIT_NUM)),
        )
        .set(dgc_platform_schema::end_commit_num.eq(current_commit_num))
        .execute(conn)
        .map(|_| ())
}

pub fn update_definition_end_commit_num(
    conn: &PgConnection,
    name: &str,
    schema_name: &str,
    current_commit_num: i64,
) -> QueryResult<()> {
    update(property_definition::table)
        .filter(
            property_definition::schema_name
                .eq(schema_name)
                .and(property_definition::name.eq(name))
                .and(property_definition::end_commit_num.eq(MAX_COMMIT_NUM)),
        )
        .set(property_definition::end_commit_num.eq(current_commit_num))
        .execute(conn)
        .map(|_| ())
}

pub fn list_grid_schemas(
    conn: &PgConnection,
    service_id: Option<&str>,
) -> QueryResult<Vec<Schema>> {
    let mut query = dgc_platform_schema::table
        .into_boxed()
        .select(dgc_platform_schema::all_columns)
        .filter(dgc_platform_schema::end_commit_num.eq(MAX_COMMIT_NUM));

    if let Some(service_id) = service_id {
        query = query.filter(dgc_platform_schema::service_id.eq(service_id));
    } else {
        query = query.filter(dgc_platform_schema::service_id.is_null());
    }

    query.load::<Schema>(conn)
}

pub fn list_grid_property_definitions(
    conn: &PgConnection,
    service_id: Option<&str>,
) -> QueryResult<Vec<PropertyDefinition>> {
    let mut query = property_definition::table
        .into_boxed()
        .select(property_definition::all_columns)
        .filter(property_definition::end_commit_num.eq(MAX_COMMIT_NUM));

    if let Some(service_id) = service_id {
        query = query.filter(property_definition::service_id.eq(service_id));
    } else {
        query = query.filter(property_definition::service_id.is_null());
    }

    query.load::<PropertyDefinition>(conn)
}

pub fn fetch_grid_schema(
    conn: &PgConnection,
    name: &str,
    service_id: Option<&str>,
) -> QueryResult<Option<Schema>> {
    let mut query = dgc_platform_schema::table
        .into_boxed()
        .select(dgc_platform_schema::all_columns)
        .filter(
            dgc_platform_schema::name
                .eq(name)
                .and(dgc_platform_schema::end_commit_num.eq(MAX_COMMIT_NUM)),
        );

    if let Some(service_id) = service_id {
        query = query.filter(dgc_platform_schema::service_id.eq(service_id));
    } else {
        query = query.filter(dgc_platform_schema::service_id.is_null());
    }

    query
        .first(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn list_grid_property_definitions_with_schema_name(
    conn: &PgConnection,
    schema_name: &str,
    service_id: Option<&str>,
) -> QueryResult<Vec<PropertyDefinition>> {
    let mut query = property_definition::table
        .into_boxed()
        .select(property_definition::all_columns)
        .filter(
            property_definition::schema_name
                .eq(schema_name)
                .and(property_definition::end_commit_num.eq(MAX_COMMIT_NUM)),
        );

    if let Some(service_id) = service_id {
        query = query.filter(property_definition::service_id.eq(service_id));
    } else {
        query = query.filter(property_definition::service_id.is_null());
    }
    query.load::<PropertyDefinition>(conn)
}
