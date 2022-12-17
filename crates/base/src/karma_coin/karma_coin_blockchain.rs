#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBlockRequest {
    #[prost(message, repeated, tag = "1")]
    pub transactions: ::prost::alloc::vec::Vec<super::core_types::SignedTransaction>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateBlockResponse {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHeadHeightRequest {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetHeadHeightResponse {
    #[prost(uint64, tag = "1")]
    pub height: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHeightRequest {
    #[prost(uint64, tag = "1")]
    pub height: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetBlockByHeightResponse {
    #[prost(message, optional, tag = "1")]
    pub block: ::core::option::Option<super::core_types::Block>,
}
#[doc = r" Generated client implementations."]
pub mod blockchain_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = " Blockchain api"]
    #[derive(Debug, Clone)]
    pub struct BlockchainServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl BlockchainServiceClient<tonic::transport::Channel> {
        #[doc = r" Attempt to create a new client by connecting to a given endpoint."]
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> BlockchainServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + Send + Sync + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> BlockchainServiceClient<InterceptedService<T, F>>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<http::Request<tonic::body::BoxBody>>>::Error:
                Into<StdError> + Send + Sync,
        {
            BlockchainServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        #[doc = r" Compress requests with `gzip`."]
        #[doc = r""]
        #[doc = r" This requires the server to support it otherwise it might respond with an"]
        #[doc = r" error."]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        #[doc = r" Enable decompressing responses with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        #[doc = " Create a block with the given transactions"]
        pub async fn create_block(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateBlockRequest>,
        ) -> Result<tonic::Response<super::CreateBlockResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.blockchain.BlockchainService/CreateBlock",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        #[doc = " Get the current blockchain tip - the height of the tip block"]
        pub async fn get_head_height(
            &mut self,
            request: impl tonic::IntoRequest<super::GetHeadHeightRequest>,
        ) -> Result<tonic::Response<super::GetHeadHeightResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.blockchain.BlockchainService/GetHeadHeight",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_block_by_height(
            &mut self,
            request: impl tonic::IntoRequest<super::GetBlockByHeightRequest>,
        ) -> Result<tonic::Response<super::GetBlockByHeightResponse>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/karma_coin.blockchain.BlockchainService/GetBlockByHeight",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
#[doc = r" Generated server implementations."]
pub mod blockchain_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with BlockchainServiceServer."]
    #[async_trait]
    pub trait BlockchainService: Send + Sync + 'static {
        #[doc = " Create a block with the given transactions"]
        async fn create_block(
            &self,
            request: tonic::Request<super::CreateBlockRequest>,
        ) -> Result<tonic::Response<super::CreateBlockResponse>, tonic::Status>;
        #[doc = " Get the current blockchain tip - the height of the tip block"]
        async fn get_head_height(
            &self,
            request: tonic::Request<super::GetHeadHeightRequest>,
        ) -> Result<tonic::Response<super::GetHeadHeightResponse>, tonic::Status>;
        async fn get_block_by_height(
            &self,
            request: tonic::Request<super::GetBlockByHeightRequest>,
        ) -> Result<tonic::Response<super::GetBlockByHeightResponse>, tonic::Status>;
    }
    #[doc = " Blockchain api"]
    #[derive(Debug)]
    pub struct BlockchainServiceServer<T: BlockchainService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: BlockchainService> BlockchainServiceServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(inner: T, interceptor: F) -> InterceptedService<Self, F>
        where
            F: FnMut(tonic::Request<()>) -> Result<tonic::Request<()>, tonic::Status>,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        #[doc = r" Enable decompressing requests with `gzip`."]
        pub fn accept_gzip(mut self) -> Self {
            self.accept_compression_encodings.enable_gzip();
            self
        }
        #[doc = r" Compress responses with `gzip`, if the client supports it."]
        pub fn send_gzip(mut self) -> Self {
            self.send_compression_encodings.enable_gzip();
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for BlockchainServiceServer<T>
    where
        T: BlockchainService,
        B: Body + Send + Sync + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/karma_coin.blockchain.BlockchainService/CreateBlock" => {
                    #[allow(non_camel_case_types)]
                    struct CreateBlockSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService>
                        tonic::server::UnaryService<super::CreateBlockRequest>
                        for CreateBlockSvc<T>
                    {
                        type Response = super::CreateBlockResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateBlockRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_block(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateBlockSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.blockchain.BlockchainService/GetHeadHeight" => {
                    #[allow(non_camel_case_types)]
                    struct GetHeadHeightSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService>
                        tonic::server::UnaryService<super::GetHeadHeightRequest>
                        for GetHeadHeightSvc<T>
                    {
                        type Response = super::GetHeadHeightResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetHeadHeightRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_head_height(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetHeadHeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/karma_coin.blockchain.BlockchainService/GetBlockByHeight" => {
                    #[allow(non_camel_case_types)]
                    struct GetBlockByHeightSvc<T: BlockchainService>(pub Arc<T>);
                    impl<T: BlockchainService>
                        tonic::server::UnaryService<super::GetBlockByHeightRequest>
                        for GetBlockByHeightSvc<T>
                    {
                        type Response = super::GetBlockByHeightResponse;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetBlockByHeightRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get_block_by_height(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetBlockByHeightSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec).apply_compression_config(
                            accept_compression_encodings,
                            send_compression_encodings,
                        );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .header("content-type", "application/grpc")
                        .body(empty_body())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: BlockchainService> Clone for BlockchainServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: BlockchainService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: BlockchainService> tonic::transport::NamedService for BlockchainServiceServer<T> {
        const NAME: &'static str = "karma_coin.blockchain.BlockchainService";
    }
}
