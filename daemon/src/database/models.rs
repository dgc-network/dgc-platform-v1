// Copyright (c) The dgc.network
// SPDX-License-Identifier: Apache-2.0

use byteorder::{NetworkEndian, ReadBytesExt};
use diesel::deserialize::{self, FromSql};
use diesel::pg::Pg;
use diesel::serialize::{self, Output, ToSql, WriteTuple};
use diesel::sql_types;
use serde_json::Value as JsonValue;
use std::io::Write;
use std::time::SystemTime;

use super::schema::{
    agent, associated_agent, commit, circuit, circuit_member, circuit_proposal,
    circuit_proposal_vote_record, property_definition, dgc_platform_schema, organization,
    product, product_property_value, property, proposal, record, reported_value, reporter,
};

#[derive(Insertable, Queryable)]
#[table_name = "commit"]
pub struct NewCommit {
    pub commit_id: String,
    pub commit_num: i64,
    pub service_id: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Commit {
    pub id: i64,
    pub commit_id: String,
    pub commit_num: i64,
    pub service_id: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "agent"]
pub struct NewAgent {
    pub public_key: String,
    pub org_id: String,
    pub active: bool,
    pub roles: Vec<String>,
    pub metadata: JsonValue,

    // The indicators of the start and stop for the slowly-changing dimensions.
    pub start_commit_num: i64,
    pub end_commit_num: i64,

    pub service_id: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Agent {
    ///  This is the record id for the slowly-changing-dimensions table.
    pub id: i64,
    // The indicators of the start and stop for the slowly-changing dimensions.
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub public_key: String,
    pub org_id: String,
    pub active: bool,
    pub roles: Vec<String>,
    pub metadata: JsonValue,
    pub service_id: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "organization"]
pub struct NewOrganization {
    pub org_id: String,
    pub name: String,
    pub address: String,
    pub metadata: Vec<JsonValue>,

    // The indicators of the start and stop for the slowly-changing dimensions.
    pub start_commit_num: i64,
    pub end_commit_num: i64,

    pub service_id: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Organization {
    ///  This is the record id for the slowly-changing-dimensions table.
    pub id: i64,
    pub org_id: String,
    pub name: String,
    pub address: String,
    pub metadata: Vec<JsonValue>,

    // The indicators of the start and stop for the slowly-changing dimensions.
    pub start_commit_num: i64,
    pub end_commit_num: i64,

    pub service_id: Option<String>,
}

#[derive(Clone, Insertable, Debug)]
#[table_name = "product"]
pub struct NewProduct {
    pub product_id: String,
    pub product_address: String,
    pub product_namespace: String,
    pub owner: String,

    // The indicators of the start and stop for the slowly-changing dimensions.
    pub start_commit_num: i64,
    pub end_commit_num: i64,

    pub service_id: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct Product {
    ///  This is the product id for the slowly-changing-dimensions table.
    pub id: i64,
    pub product_id: String,
    pub product_address: String,
    pub product_namespace: String,
    pub owner: String,

    // The indicators of the start and stop for the slowly-changing dimensions.
    pub start_commit_num: i64,
    pub end_commit_num: i64,

    pub service_id: Option<String>,
}

#[derive(Clone, Insertable, Debug)]
#[table_name = "product_property_value"]
pub struct NewProductPropertyValue {
    pub product_id: String,
    pub product_address: String,
    pub property_name: String,
    pub data_type: String,
    pub bytes_value: Option<Vec<u8>>,
    pub boolean_value: Option<bool>,
    pub number_value: Option<i64>,
    pub string_value: Option<String>,
    pub enum_value: Option<i32>,
    pub struct_values: Option<Vec<String>>,
    pub lat_long_value: Option<LatLongValue>,

    // The indicators of the start and stop for the slowly-changing dimensions.
    pub start_commit_num: i64,
    pub end_commit_num: i64,

    pub service_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Queryable, Debug)]
pub struct ProductPropertyValue {
    ///  This is the product id for the slowly-changing-dimensions table.
    pub id: i64,
    pub product_id: String,
    pub product_address: String,
    pub property_name: String,
    pub data_type: String,
    pub bytes_value: Option<Vec<u8>>,
    pub boolean_value: Option<bool>,
    pub number_value: Option<i64>,
    pub string_value: Option<String>,
    pub enum_value: Option<i32>,
    pub struct_values: Option<Vec<String>>,
    pub lat_long_value: Option<LatLongValue>,

    // The indicators of the start and stop for the slowly-changing dimensions.
    pub start_commit_num: i64,
    pub end_commit_num: i64,

    pub service_id: Option<String>,
}

#[derive(Clone, Insertable, Debug)]
#[table_name = "dgc_platform_schema"]
pub struct NewSchema {
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub service_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Queryable, Debug)]
pub struct Schema {
    pub id: i64,
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub name: String,
    pub description: String,
    pub owner: String,
    pub service_id: Option<String>,
}

#[derive(Clone, Insertable, Debug)]
#[table_name = "property_definition"]
pub struct NewPropertyDefinition {
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub name: String,
    pub schema_name: String,
    pub data_type: String,
    pub required: bool,
    pub description: String,
    pub number_exponent: i64,
    pub enum_options: Vec<String>,
    pub struct_properties: Vec<String>,
    pub service_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Queryable, Debug)]
pub struct PropertyDefinition {
    pub id: i64,
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub name: String,
    pub schema_name: String,
    pub data_type: String,
    pub required: bool,
    pub description: String,
    pub number_exponent: i64,
    pub enum_options: Vec<String>,
    pub struct_properties: Vec<String>,
    pub service_id: Option<String>,
}

#[derive(SqlType, QueryId, Debug, Clone, Copy)]
#[postgres(type_name = "latlong")]
pub struct LatLong;

#[derive(Debug, PartialEq, FromSqlRow, AsExpression, Clone)]
#[sql_type = "LatLong"]
pub struct LatLongValue(pub i64, pub i64);

impl ToSql<LatLong, Pg> for LatLongValue {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        WriteTuple::<(sql_types::BigInt, sql_types::BigInt)>::write_tuple(&(self.0, self.1), out)
    }
}

impl FromSql<LatLong, Pg> for LatLongValue {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        let mut bytes = not_none!(bytes);
        let num_elements = bytes.read_i32::<NetworkEndian>()?;
        if num_elements != 2 {
            return Err(format!("Expected a tuple of 2 elements, got {}", num_elements,).into());
        }
        let (_, mut bytes) = bytes.split_at(std::mem::size_of::<i32>() * 2);
        let lat = bytes.read_i64::<NetworkEndian>()?;
        let (_, mut bytes) = bytes.split_at(std::mem::size_of::<i32>() * 2);
        let long = bytes.read_i64::<NetworkEndian>()?;
        Ok(LatLongValue(lat, long))
    }
}

#[derive(Insertable, Debug)]
#[table_name = "associated_agent"]
pub struct NewAssociatedAgent {
    pub record_id: String,
    pub role: String,
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub agent_id: String,
    pub timestamp: i64,
    pub service_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Queryable, Debug, Clone)]
pub struct AssociatedAgent {
    pub id: i64,
    pub record_id: String,
    pub role: String,
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub agent_id: String,
    pub timestamp: i64,
    pub service_id: Option<String>,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "property"]
pub struct NewProperty {
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub name: String,
    pub record_id: String,
    pub property_definition: String,
    pub current_page: i32,
    pub wrapped: bool,
    pub service_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Queryable, Debug, Clone)]
pub struct Property {
    pub id: i64,
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub name: String,
    pub record_id: String,
    pub property_definition: String,
    pub current_page: i32,
    pub wrapped: bool,
    pub service_id: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "proposal"]
pub struct NewProposal {
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub record_id: String,
    pub timestamp: i64,
    pub issuing_agent: String,
    pub receiving_agent: String,
    pub role: String,
    pub properties: Vec<String>,
    pub status: String,
    pub terms: String,
    pub service_id: Option<String>,
}

#[derive(Queryable, Debug, Clone)]
pub struct Proposal {
    pub id: i64,
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub record_id: String,
    pub timestamp: i64,
    pub issuing_agent: String,
    pub receiving_agent: String,
    pub role: String,
    pub properties: Vec<String>,
    pub status: String,
    pub terms: String,
    pub service_id: Option<String>,
}

#[derive(Insertable, Debug)]
#[table_name = "record"]
pub struct NewRecord {
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub record_id: String,
    pub schema: String,
    pub final_: bool,
    pub owners: Vec<String>,
    pub custodians: Vec<String>,
    pub service_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Queryable, Debug)]
pub struct Record {
    pub id: i64,
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub record_id: String,
    pub schema: String,
    pub final_: bool,
    pub owners: Vec<String>,
    pub custodians: Vec<String>,
    pub service_id: Option<String>,
}

#[derive(Insertable, Debug, Clone, Default)]
#[table_name = "reported_value"]
pub struct NewReportedValue {
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub property_name: String,
    pub record_id: String,
    pub reporter_index: i32,
    pub timestamp: i64,
    pub data_type: String,
    pub bytes_value: Option<Vec<u8>>,
    pub boolean_value: Option<bool>,
    pub number_value: Option<i64>,
    pub string_value: Option<String>,
    pub enum_value: Option<i32>,
    pub struct_values: Option<Vec<String>>,
    pub lat_long_value: Option<LatLongValue>,
    pub service_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Queryable, Debug)]
pub struct ReportedValue {
    pub id: i64,
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub property_name: String,
    pub record_id: String,
    pub reporter_index: i32,
    pub timestamp: i64,
    pub data_type: String,
    pub bytes_value: Option<Vec<u8>>,
    pub boolean_value: Option<bool>,
    pub number_value: Option<i64>,
    pub string_value: Option<String>,
    pub enum_value: Option<i32>,
    pub struct_values: Option<Vec<String>>,
    pub lat_long_value: Option<LatLongValue>,
    pub service_id: Option<String>,
}

#[derive(Insertable, Debug, Clone)]
#[table_name = "reporter"]
pub struct NewReporter {
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub property_name: String,
    pub record_id: String,
    pub public_key: String,
    pub authorized: bool,
    pub reporter_index: i32,
    pub service_id: Option<String>,
}

#[allow(dead_code)]
#[derive(Queryable, Debug)]
pub struct Reporter {
    pub id: i64,
    pub start_commit_num: i64,
    pub end_commit_num: i64,
    pub property_name: String,
    pub record_id: String,
    pub public_key: String,
    pub authorized: bool,
    pub reporter_index: i32,
    pub service_id: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct ReportedValueReporterToAgentMetadata {
    pub id: i64,
    pub property_name: String,
    pub record_id: String,
    pub reporter_index: i32,
    pub timestamp: i64,
    pub data_type: String,
    pub bytes_value: Option<Vec<u8>>,
    pub boolean_value: Option<bool>,
    pub number_value: Option<i64>,
    pub string_value: Option<String>,
    pub enum_value: Option<i32>,
    pub struct_values: Option<Vec<String>>,
    pub lat_long_value: Option<LatLongValue>,
    pub public_key: Option<String>,
    pub authorized: Option<bool>,
    pub metadata: Option<JsonValue>,
    pub reported_value_end_commit_num: i64,
    pub reporter_end_commit_num: Option<i64>,
    pub service_id: Option<String>,
}

#[derive(Insertable, Queryable, Identifiable, PartialEq, Debug)]
#[table_name = "circuit"]
#[primary_key(circuit_id)]
pub struct Circuit {
    pub circuit_id: String,
    pub authorization_type: String,
    pub persistence: String,
    pub durability: String,
    pub routes: String,
    pub circuit_management_type: String,
    pub alias: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "circuit_proposal"]
#[belongs_to(Circuit, foreign_key = "circuit_id")]
pub struct CircuitProposal {
    pub id: i64,
    pub proposal_type: String,
    pub circuit_id: String,
    pub circuit_hash: String,
    pub requester: String,
    pub requester_node_id: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "circuit_proposal"]
pub struct NewCircuitProposal {
    pub proposal_type: String,
    pub circuit_id: String,
    pub circuit_hash: String,
    pub requester: String,
    pub requester_node_id: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "circuit_member"]
#[belongs_to(Circuit, foreign_key = "circuit_id")]
pub struct CircuitMember {
    pub id: i64,
    pub circuit_id: String,
    pub node_id: String,
    pub endpoint: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "circuit_member"]
pub struct NewCircuitMember {
    pub circuit_id: String,
    pub node_id: String,
    pub endpoint: String,
    pub status: String,
    pub created_time: SystemTime,
    pub updated_time: SystemTime,
}

#[derive(Queryable, Identifiable, Associations, PartialEq, Debug)]
#[table_name = "circuit_proposal_vote_record"]
#[belongs_to(CircuitProposal, foreign_key = "proposal_id")]
pub struct CircuitProposalVoteRecord {
    pub id: i64,
    pub proposal_id: i64,
    pub voter_public_key: String,
    pub voter_node_id: String,
    pub vote: String,
    pub created_time: SystemTime,
}

#[derive(Insertable, PartialEq, Debug)]
#[table_name = "circuit_proposal_vote_record"]
pub struct NewCircuitProposalVoteRecord {
    pub proposal_id: i64,
    pub voter_public_key: String,
    pub voter_node_id: String,
    pub vote: String,
    pub created_time: SystemTime,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserializing_lat_long_value_from_bytes() {
        let lat_long_bytes = [
            0, 0, 0, 2, 0, 0, 0, 20, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 20, 0, 0, 0, 8,
            255, 255, 255, 255, 255, 255, 255, 255,
        ];
        let lat_long = LatLongValue::from_sql(Some(&lat_long_bytes))
            .expect("Failed to deserialize LatLongValue");

        assert_eq!(lat_long, LatLongValue(0, -1));
    }
}
