// Copyright (c) 2022, KarmaCoin Authors. a@karmaco.in.
// This work is licensed under the KarmaCoin v0.1.0 license published in the LICENSE file of this repo.
//

use anyhow::Result;
use base::karma_coin::karma_coin_api::api_service_server::ApiService as ApiServiceTrait;
use tonic::{Request, Response, Status};
use base::karma_coin::karma_coin_api::{GetBlockchainEventsRequest, GetBlockchainEventsResponse,
                                       GetCharTraitsRequest, GetCharTraitsResponse,
                                       GetNetInfoRequest, GetNetInfoResponse, GetPhoneVerifiersRequest,
                                       GetPhoneVerifiersResponse, GetTransactionRequest, GetTransactionResponse,
                                       GetTransactionsRequest, GetTransactionsResponse, GetUserInfoByAccountRequest,
                                       GetUserInfoByAccountResponse, GetUserInfoByNickRequest,
                                       GetUserInfoByNickResponse, GetUserInfoByNumberRequest,
                                       GetUserInfoByNumberResponse, SubmitTransactionRequest,
                                       SubmitTransactionResponse};
use xactor::*;
use crate::services::api::get_char_traits::GetCharTraits;
use crate::services::api::get_user_by_account_id::GetUserInfoByAccountId;
use crate::services::api::get_user_by_nick::GetUserInfoByNick;
use crate::services::api::get_user_by_number::GetUserInfoByNumber;

/// ApiService is a system service that provides access to provider server persisted data as well as an interface to admin the provider's server. It provides a GRPC admin service defined in ServerAdminService. This service is designed to be used by provider admin clients.
#[derive(Debug, Clone)]
pub(crate) struct ApiService {}

impl Default for ApiService {
    fn default() -> Self {
        info!("Api Service created");
        ApiService {}
    }
}

#[async_trait::async_trait]
impl Actor for ApiService {
    async fn started(&mut self, _ctx: &mut Context<Self>) -> Result<()> {
        info!("ApiService started");
        Ok(())
    }
}

impl Service for ApiService {}


/// ApiService implements the ApiServiceTrait trait which defines the grpc rpc methods it provides for clients over the network
#[tonic::async_trait]
impl ApiServiceTrait for ApiService {

    /// Returns user info by nickname
    async fn get_user_info_by_nick(&self, request: Request<GetUserInfoByNickRequest>) -> Result<Response<GetUserInfoByNickResponse>, Status> {

        let service = ApiService::from_registry().await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?;

        let res = service.call(GetUserInfoByNick(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?
            .map_err(|_| Status::internal("internal error"))?;

        Ok(Response::new(res))
    }

    /// Returns user info by verified mobile phone number
    async fn get_user_info_by_number(
        &self,
        request: Request<GetUserInfoByNumberRequest>,
    ) -> std::result::Result<Response<GetUserInfoByNumberResponse>, Status> {

        let service = ApiService::from_registry().await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?;

        let res = service.call(GetUserInfoByNumber(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?
            .map_err(|_| Status::internal("internal error"))?;

        Ok(Response::new(res))
    }

    /// Returns user info by his unique account id
    async fn get_user_info_by_account(
        &self,
        request: Request<GetUserInfoByAccountRequest>,
    ) -> std::result::Result<Response<GetUserInfoByAccountResponse>, Status> {

        let service = ApiService::from_registry().await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?;

        let res = service.call(GetUserInfoByAccountId(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?
            .map_err(|_| Status::internal("internal error"))?;

        Ok(Response::new(res))
    }

    /// Returns supported phone verifiers identity (on-chain data)
    async fn get_phone_verifiers(
        &self,
        _request: Request<GetPhoneVerifiersRequest>,
    ) -> std::result::Result<Response<GetPhoneVerifiersResponse>, Status> {
        todo!()
    }

    /// Returns the supported character traits
    async fn get_char_traits(
        &self,
        request: Request<GetCharTraitsRequest>,
    ) -> std::result::Result<Response<GetCharTraitsResponse>, Status> {

        let service = ApiService::from_registry().await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?;

        let res = service.call(GetCharTraits(request.into_inner()))
            .await
            .map_err(|e| Status::internal(format!("failed to call api: {:?}", e)))?
            .map_err(|_| Status::internal("internal error"))?;

        Ok(Response::new(res))

    }

    /// Returns network info (on-chain data)
    async fn get_net_info(
        &self,
        _request: Request<GetNetInfoRequest>,
    ) -> std::result::Result<Response<GetNetInfoResponse>, Status> {
        todo!()
    }

    /// Submit a transaction for processing
    async fn submit_transaction(&self,_request: Request<SubmitTransactionRequest>) -> std::result::Result<Response<SubmitTransactionResponse>, Status> {
        todo!()
    }

    /// Returns all transactions to, and or from and account
    async fn get_transactions(
        &self,
        _request: Request<GetTransactionsRequest>,
    ) -> Result<Response<GetTransactionsResponse>, Status> {
        todo!()
    }

    /// Get a signed transaction by its hash
    async fn get_transaction(
        &self,
        _request: Request<GetTransactionRequest>,
    ) -> Result<Response<GetTransactionResponse>, Status> {
        todo!()
    }

    /// Returns all transactions processing events from a block height to a block height
    async fn get_blockchain_events(
        &self,
        _request: Request<GetBlockchainEventsRequest>,
    ) -> Result<Response<GetBlockchainEventsResponse>, Status> {
        todo!()
    }
}
