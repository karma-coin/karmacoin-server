// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use crate::services::blockchain::blockchain_service::BlockChainService;
use crate::services::db_config_service::{
    TRANSACTIONS_COL_FAMILY, TRANSACTIONS_EVENTS_COL_FAMILY,
    TRANSACTIONS_HASHES_BY_ACCOUNT_IDX_COL_FAMILY,
};
use anyhow::Result;
use base::karma_coin::karma_coin_core_types::TransactionStatus::OnChain;
use base::karma_coin::karma_coin_core_types::*;
use bytes::Bytes;
use db::db_service::{DataItem, DatabaseService, ReadItem, WriteItem};
use prost::Message;
use xactor::*;

#[message(result = "Result<TransactionEvents>")]
pub(crate) struct GetTransactionEvents {
    pub(crate) tx_hash: Bytes,
}

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<GetTransactionEvents> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetTransactionEvents,
    ) -> Result<TransactionEvents> {
        self.get_tx_events(msg.tx_hash).await
    }
}

#[message(result = "Result<Vec<SignedTransactionWithStatus>>")]
pub(crate) struct GetTransactionsByAccountId {
    pub(crate) account_id: Bytes,
}

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<GetTransactionsByAccountId> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetTransactionsByAccountId,
    ) -> Result<Vec<SignedTransactionWithStatus>> {
        self.get_transactions_by_account_id(msg.account_id).await
    }
}

#[message(result = "Result<Option<SignedTransactionWithStatus>>")]
pub(crate) struct GetTransactionByHash {
    pub(crate) hash: Bytes,
}

/// Request to complete verification and sign up
#[async_trait::async_trait]
impl Handler<GetTransactionByHash> for BlockChainService {
    async fn handle(
        &mut self,
        _ctx: &mut Context<Self>,
        msg: GetTransactionByHash,
    ) -> Result<Option<SignedTransactionWithStatus>> {
        self.get_transaction_by_hash(msg.hash).await
    }
}

impl BlockChainService {
    /// Get all translation events for a given transaction hash
    pub(crate) async fn get_tx_events(&self, tx_hash: Bytes) -> Result<TransactionEvents> {
        if let Some(data) = DatabaseService::read(ReadItem {
            key: tx_hash,
            cf: TRANSACTIONS_EVENTS_COL_FAMILY,
        })
        .await?
        {
            Ok(TransactionEvents::decode(data.0.as_ref())?)
        } else {
            Ok(TransactionEvents::default())
        }
    }

    pub(crate) async fn get_transaction_by_hash(
        &self,
        hash: Bytes,
    ) -> Result<Option<SignedTransactionWithStatus>> {
        if let Some(data) = DatabaseService::read(ReadItem {
            key: hash,
            cf: TRANSACTIONS_COL_FAMILY,
        })
        .await?
        {
            let tx = SignedTransaction::decode(data.0.as_ref())?;
            Ok(Some(SignedTransactionWithStatus {
                transaction: Some(tx),
                status: OnChain as i32,
            }))
        } else {
            Ok(None)
        }
    }

    /// Get transactions by account id
    /// These include all transactions to and from this account
    pub(crate) async fn get_transactions_by_account_id(
        &self,
        account_id: Bytes,
    ) -> Result<Vec<SignedTransactionWithStatus>> {
        return if let Some(data) = DatabaseService::read(ReadItem {
            key: account_id,
            cf: TRANSACTIONS_HASHES_BY_ACCOUNT_IDX_COL_FAMILY,
        })
        .await?
        {
            let data = SignedTransactionsHashes::decode(data.0.as_ref())?;
            let mut txs = vec![];

            for tx_hash in data.hashes {
                if let Some(data) = DatabaseService::read(ReadItem {
                    key: Bytes::from(tx_hash),
                    cf: TRANSACTIONS_COL_FAMILY,
                })
                .await?
                {
                    let tx = SignedTransaction::decode(data.0.as_ref())?;
                    txs.push(SignedTransactionWithStatus {
                        transaction: Some(tx),
                        status: OnChain as i32,
                    });
                }
            }
            Ok(txs)
        } else {
            Ok(vec![])
        };
    }

    /// Index a transaction by an account id
    pub(crate) async fn index_transaction_by_account_id(
        &mut self,
        transaction: &SignedTransaction,
        account_id: Bytes,
    ) -> Result<()> {
        let tx_hash = transaction.get_hash()?;

        let tx_hashes = if let Some(data) = DatabaseService::read(ReadItem {
            key: account_id.clone(),
            cf: TRANSACTIONS_HASHES_BY_ACCOUNT_IDX_COL_FAMILY,
        })
        .await?
        {
            let mut data = SignedTransactionsHashes::decode(data.0.as_ref())?;
            data.hashes.push(tx_hash.clone().to_vec());
            data
        } else {
            SignedTransactionsHashes {
                hashes: vec![tx_hash.clone().to_vec()],
            }
        };

        let mut buf = Vec::with_capacity(tx_hashes.encoded_len());
        tx_hashes.encode(&mut buf)?;

        DatabaseService::write(WriteItem {
            data: DataItem {
                key: account_id,
                value: Bytes::from(buf),
            },
            cf: TRANSACTIONS_HASHES_BY_ACCOUNT_IDX_COL_FAMILY,
            ttl: 0,
        })
        .await
    }
}
