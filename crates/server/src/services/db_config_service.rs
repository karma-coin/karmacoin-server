// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use rocksdb::{ColumnFamilyDescriptor, Options};
use base::server_config_service::{DB_NAME_CONFIG_KEY, DROP_DB_CONFIG_KEY, ServerConfigService};
use db::db_service::{DatabaseService, DataItem, ReadItem, WriteItem};
use xactor::*;
use anyhow::{anyhow, Result};
use bytes::Bytes;
use base::karma_coin::karma_coin_core_types::CharTrait::{Helpful, Kind, Smart};
use base::karma_coin::karma_coin_core_types::{TraitName, Traits};

/// db data modeling - column families and their stored data
/// Encoding conventions:
/// - string keys are utf8 encoded to bytes
/// - Numbers are LittleEndian encoded
/// - Protobuf objects are serialized using prost

////
// Verier local data
//////////////

// Tracking codes sent to new users before they are users
// index: verification_code, data: accountId. ttl: 24 hours
pub const VERIFICATION_CODES_COL_FAMILY: &str = "verification_codes_cf";

// Unique reserved nicks (bin-coded strings). data: accountId. ttl: 24 hours
// Nicks are reserved by new users when they verify their phone so they can claim the nicks
// in up to 24 hours from verification via the CreateUser transaction
pub const RESERVED_NICKS_COL_FAMILY: &str = "reserved_nicks_cf";


/////
//// Blockchain-based data - indexing on-chain data and its blocks
/////////////////

// col family the network settings. Various settings are accessible via keys.
pub const NET_SETTINGS_COL_FAMILY: &str = "net_settings_cf";

// value: bool indicating if the local db was initialized or needs initiation with static data
pub const DB_INITIALIZED_KEY: &str = "db_initialized_key";

// Value: a serialized vector of all supported Traits
// this data is in consensus on genesis and may only change via a runtime upgrade
pub const DB_SUPPORTED_TRAITS_KEY: &str = "supported_traits_key";

// col family for verifiers on-chain data. index: accountId, data: Verifier dial-up info
// this data is in consensus on genesis and can only change via a runtime update
pub const VERIFIERS_COL_FAMILY: &str = "verifiers_cf";

// A mapping of account ids to users. key: accountId, data: User
// This is on-chain data.
// All users accounts in consensus on-chain.
pub const USERS_COL_FAMILY: &str = "users_cf";

// A mapping of nicknames to account ids.
// This is on-chain data derived from on-chain users accounts data.
// key: nickname (utf8 encoded string). value: accountId.
pub const NICKS_COL_FAMILY: &str = "nicks_cf";

// A mapping from mobile phone numbers to registered users.
// This is on-chain data derived from on-chain users accounts data.
// key: mobile number (utf-8 encoded). value: accountId
pub const MOBILE_NUMBERS_COL_FAMILY: &str = "mobile_number_cf";

// Signed transactions indexed by their hash. Data: SignTransaction
// This is on-chain data
pub const TRANSACTIONS_COL_FAMILY: &str = "txs_cf";

// Blocks keyed by block number - the blockchain. index: block height. value: Block
// This is the actual blockchain
pub const BLOCKS_COL_FAMILY: &str = "blocks_cf";

// Valid transactions submitted to the chain, not yet processed and queued in the txs pool
// This is off-chain tx pool data
pub const TXS_POOL_COL_FAMILY: &str = "txs_mem_pool_cf";

// Used for db testing - doesn't hold any app data
pub const TESTS_COL_FAMILY: &str = "tests_cf"; // col family for db tests



#[derive(Debug, Clone)]
pub(crate) struct DbConfigService {
}

impl Default for DbConfigService {
    fn default() -> Self {
        info!("DbConfigService Service started");
        DbConfigService {}
    }
}

#[async_trait::async_trait]
impl Actor for DbConfigService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("Configuring the db...");

        let db_name = ServerConfigService::get(DB_NAME_CONFIG_KEY.into())
            .await?
            .unwrap();

        let drop_on_exit = ServerConfigService::get_bool(DROP_DB_CONFIG_KEY.into())
            .await?
            .unwrap();

        // configure the db
        DatabaseService::config_db(db::db_service::Configure {
            drop_on_exit,
            db_name,
            col_descriptors: vec![
                ColumnFamilyDescriptor::new(VERIFIERS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(USERS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(NICKS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(MOBILE_NUMBERS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(VERIFICATION_CODES_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(NET_SETTINGS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TESTS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(BLOCKS_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TXS_POOL_COL_FAMILY, Options::default()),
                ColumnFamilyDescriptor::new(TRANSACTIONS_COL_FAMILY, Options::default()),
            ],
        }).await?;

        // cehck if db was initialized with static net-specific data
        let init_key = Bytes::from(DB_INITIALIZED_KEY.as_bytes());
        if DatabaseService::read(ReadItem {
            key: init_key.clone(),
            cf: NET_SETTINGS_COL_FAMILY
        }).await?.is_none() {
            // initialize db static GENESIS data here
            let traits = Traits {
                // todo: traits should come from config file
                named_traits: vec![
                    TraitName::new(Kind, "Kind"),
                    TraitName::new(Helpful, "Helpful"),
                    TraitName::new(Smart, "Smart"),
                ]
            };

            use prost::Message;
            let mut buf = Vec::with_capacity(traits.encoded_len());
            if traits.encode(&mut buf).is_err() {
                return Err(anyhow!("failed to encode default traits"));
            };

            // store default char traits
            DatabaseService::write(WriteItem {
                data: DataItem { key: Bytes::from(DB_SUPPORTED_TRAITS_KEY.as_bytes()),
                    value: Bytes::from(buf) },
                cf: NET_SETTINGS_COL_FAMILY,
                ttl: 0
            }).await?;

        /*
            todo: initialize these settings - genesis config:

            uint32 network_id = 1;
            uint64 users_count = 2;
            uint64 genesis_time = 3;
            string name = 4;
            uint64 block_height = 5;
            string api_version = 6; // provided API semantic version
            uint64 transactions_count = 7; // number of transactions
            uint64 appreciations_count = 8; // number of appreciations
            uint64 new_account_reward = 9; // new account reward in kcents
            uint64 referral_reward = 10; // referral reward in kcents
        */

        }

        // mark that db is configured with static data
        DatabaseService::write(WriteItem {
            data: DataItem {
                key: init_key,
                value: Bytes::from("1".as_bytes().to_vec()),
             },
            cf: NET_SETTINGS_COL_FAMILY,
            ttl: 0
        }).await?;

        Ok(())
    }
}

impl Service for DbConfigService {}
