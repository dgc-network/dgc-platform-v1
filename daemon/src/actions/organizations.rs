// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

//use crate::error::CliError;
use crate::actions::error::CliError;
use crate::http::submit_batches;
use crate::transaction::{pike_batch_builder, PIKE_NAMESPACE};
use grid_sdk::{
    protocol::pike::payload::{
        Action, CreateOrganizationAction, PikePayloadBuilder, UpdateOrganizationAction,
    },
    protos::IntoProto,
};

pub fn do_create_organization(
    url: &str,
    key: Option<String>,
    wait: u64,
    create_org: CreateOrganizationAction,
    service_id: Option<String>,
) -> Result<(), CliError> {
    let payload = PikePayloadBuilder::new()
        .with_action(Action::CreateOrganization)
        .with_create_organization(create_org)
        .build()
        .map_err(|err| CliError::UserError(format!("{}", err)))?;

    let batch_list = pike_batch_builder(key)
        .add_transaction(
            &payload.into_proto()?,
            &[PIKE_NAMESPACE.to_string()],
            &[PIKE_NAMESPACE.to_string()],
        )?
        .create_batch_list();

    submit_batches(url, wait, &batch_list, service_id.as_deref())
}

pub fn do_update_organization(
    url: &str,
    key: Option<String>,
    wait: u64,
    update_org: UpdateOrganizationAction,
    service_id: Option<String>,
) -> Result<(), CliError> {
    let payload = PikePayloadBuilder::new()
        .with_action(Action::UpdateOrganization)
        .with_update_organization(update_org)
        .build()
        .map_err(|err| CliError::UserError(format!("{}", err)))?;

    let batch_list = pike_batch_builder(key)
        .add_transaction(
            &payload.into_proto()?,
            &[PIKE_NAMESPACE.to_string()],
            &[PIKE_NAMESPACE.to_string()],
        )?
        .create_batch_list();

    submit_batches(url, wait, &batch_list, service_id.as_deref())
}
