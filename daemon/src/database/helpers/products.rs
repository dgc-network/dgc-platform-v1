// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use super::models::{NewProduct, NewProductPropertyValue, Product, ProductPropertyValue};
use super::schema::{product, product_property_value};
use super::MAX_COMMIT_NUM;

use diesel::{
    dsl::{insert_into, update},
    pg::PgConnection,
    prelude::*,
    result::Error::NotFound,
    QueryResult,
};

pub fn insert_products(conn: &PgConnection, products: &[NewProduct]) -> QueryResult<()> {
    for prod in products {
        update_prod_end_commit_num(conn, &prod.product_id, prod.start_commit_num)?;
    }

    insert_into(product::table)
        .values(products)
        .execute(conn)
        .map(|_| ())
}

pub fn insert_product_property_values(
    conn: &PgConnection,
    property_values: &[NewProductPropertyValue],
) -> QueryResult<()> {
    for value in property_values {
        update_prod_property_values(conn, &value.product_id, value.start_commit_num)?;
    }

    insert_into(product_property_value::table)
        .values(property_values)
        .execute(conn)
        .map(|_| ())
}

pub fn delete_product(
    conn: &PgConnection,
    address: &str,
    current_commit_num: i64,
) -> QueryResult<()> {
    update(product::table)
        .filter(
            product::product_address
                .eq(address)
                .and(product::end_commit_num.eq(MAX_COMMIT_NUM)),
        )
        .set(product::end_commit_num.eq(current_commit_num))
        .execute(conn)
        .map(|_| ())
}

pub fn delete_product_property_values(
    conn: &PgConnection,
    address: &str,
    current_commit_num: i64,
) -> QueryResult<()> {
    update(product_property_value::table)
        .filter(
            product_property_value::product_address
                .eq(address)
                .and(product_property_value::end_commit_num.eq(MAX_COMMIT_NUM)),
        )
        .set(product_property_value::end_commit_num.eq(current_commit_num))
        .execute(conn)
        .map(|_| ())
}

fn update_prod_end_commit_num(
    conn: &PgConnection,
    product_id: &str,
    current_commit_num: i64,
) -> QueryResult<()> {
    update(product::table)
        .filter(
            product::product_id
                .eq(product_id)
                .and(product::end_commit_num.eq(MAX_COMMIT_NUM)),
        )
        .set(product::end_commit_num.eq(current_commit_num))
        .execute(conn)
        .map(|_| ())
}

fn update_prod_property_values(
    conn: &PgConnection,
    product_id: &str,
    current_commit_num: i64,
) -> QueryResult<()> {
    update(product_property_value::table)
        .filter(
            product_property_value::product_id
                .eq(product_id)
                .and(product_property_value::end_commit_num.eq(MAX_COMMIT_NUM)),
        )
        .set(product_property_value::end_commit_num.eq(current_commit_num))
        .execute(conn)
        .map(|_| ())
}

pub fn list_products(conn: &PgConnection, service_id: Option<&str>) -> QueryResult<Vec<Product>> {
    let mut query = product::table
        .into_boxed()
        .select(product::all_columns)
        .filter(product::end_commit_num.eq(MAX_COMMIT_NUM));

    if let Some(service_id) = service_id {
        query = query.filter(product::service_id.eq(service_id));
    } else {
        query = query.filter(product::service_id.is_null());
    }
    query.load::<Product>(conn)
}

pub fn list_product_property_values(
    conn: &PgConnection,
    service_id: Option<&str>,
) -> QueryResult<Vec<ProductPropertyValue>> {
    let mut query = product_property_value::table
        .into_boxed()
        .select(product_property_value::all_columns)
        .filter(product_property_value::end_commit_num.eq(MAX_COMMIT_NUM));

    if let Some(service_id) = service_id {
        query = query.filter(product_property_value::service_id.eq(service_id));
    } else {
        query = query.filter(product_property_value::service_id.is_null());
    }
    query.load::<ProductPropertyValue>(conn)
}

pub fn fetch_product(
    conn: &PgConnection,
    product_id: &str,
    service_id: Option<&str>,
) -> QueryResult<Option<Product>> {
    let mut query = product::table
        .into_boxed()
        .select(product::all_columns)
        .filter(
            product::product_id
                .eq(product_id)
                .and(product::end_commit_num.eq(MAX_COMMIT_NUM)),
        );

    if let Some(service_id) = service_id {
        query = query.filter(product::service_id.eq(service_id));
    } else {
        query = query.filter(product::service_id.is_null());
    }

    query
        .first(conn)
        .map(Some)
        .or_else(|err| if err == NotFound { Ok(None) } else { Err(err) })
}

pub fn fetch_product_property_values(
    conn: &PgConnection,
    product_id: &str,
    service_id: Option<&str>,
) -> QueryResult<Vec<ProductPropertyValue>> {
    let mut query = product_property_value::table
        .into_boxed()
        .select(product_property_value::all_columns)
        .filter(
            product_property_value::product_id
                .eq(product_id)
                .and(product_property_value::end_commit_num.eq(MAX_COMMIT_NUM)),
        );

    if let Some(service_id) = service_id {
        query = query.filter(product_property_value::service_id.eq(service_id));
    } else {
        query = query.filter(product_property_value::service_id.is_null());
    }
    query.load::<ProductPropertyValue>(conn)
}
