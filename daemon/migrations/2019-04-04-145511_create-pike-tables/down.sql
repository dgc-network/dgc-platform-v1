-- Copyright 2019 dgc.network
--
-- Licensed under the Apache License, Version 2.0 (the "License");
-- you may not use this file except in compliance with the License.
-- You may obtain a copy of the License at
--
--     http://www.apache.org/licenses/LICENSE-2.0
--
-- Unless required by applicable law or agreed to in writing, software
-- distributed under the License is distributed on an "AS IS" BASIS,
-- WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
-- See the License for the specific language governing permissions and
-- limitations under the License.
-- -----------------------------------------------------------------------------

DROP TABLE IF EXISTS agent;
DROP INDEX IF EXISTS agent_pub_key_block_num_idx;

DROP TABLE IF EXISTS organization;
DROP INDEX IF EXISTS org_id_block_num_idx;

DROP TABLE IF EXISTS chain_record;
