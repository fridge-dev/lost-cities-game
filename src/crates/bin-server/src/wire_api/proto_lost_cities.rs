// =======================================
// API Request and Reply messages
// =======================================

// Convention: ALL messages should have prefix "Proto" so in the rust src, it's easy
// to understand which types are generated.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoHostGameReq {
    #[prost(string, tag = "1")]
    pub game_id: std::string::String,
    #[prost(string, tag = "2")]
    pub player_id: std::string::String,
}
/// Nothing
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoHostGameReply {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoJoinGameReq {
    #[prost(string, tag = "1")]
    pub game_id: std::string::String,
    #[prost(string, tag = "2")]
    pub player_id: std::string::String,
}
/// Nothing
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoJoinGameReply {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGetGameStateReq {
    #[prost(string, tag = "1")]
    pub game_id: std::string::String,
    #[prost(string, tag = "2")]
    pub player_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGetGameStateReply {
    #[prost(message, optional, tag = "1")]
    pub game: ::std::option::Option<ProtoGame>,
    #[prost(string, tag = "2")]
    pub opponent_player_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoPlayCardReq {
    #[prost(string, tag = "1")]
    pub game_id: std::string::String,
    #[prost(string, tag = "2")]
    pub player_id: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub card: ::std::option::Option<ProtoCard>,
    #[prost(enumeration = "ProtoPlayTarget", tag = "4")]
    pub target: i32,
    #[prost(enumeration = "ProtoDrawPile", tag = "5")]
    pub draw_pile: i32,
    #[prost(enumeration = "ProtoColor", tag = "6")]
    pub discard_draw_color: i32,
}
/// Nothing
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoPlayCardReply {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoDescribeGameReq {
    #[prost(string, tag = "1")]
    pub game_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoDescribeGameReply {
    #[prost(message, optional, tag = "1")]
    pub metadata: ::std::option::Option<ProtoGameMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoQueryGamesReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
    #[prost(enumeration = "ProtoGameStatus", tag = "2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoQueryGamesReply {
    #[prost(message, repeated, tag = "1")]
    pub games: ::std::vec::Vec<ProtoGameMetadata>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGetMatchableGamesReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGetMatchableGamesReply {
    #[prost(message, repeated, tag = "1")]
    pub games: ::std::vec::Vec<ProtoGameMetadata>,
}
// =======================================
// Sub types
// =======================================

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoCard {
    #[prost(enumeration = "ProtoColor", tag = "1")]
    pub color: i32,
    #[prost(uint32, tag = "2")]
    pub value: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoPlayHistory {
    #[prost(uint32, repeated, tag = "1")]
    pub red: ::std::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "2")]
    pub green: ::std::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "3")]
    pub white: ::std::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "4")]
    pub blue: ::std::vec::Vec<u32>,
    #[prost(uint32, repeated, tag = "5")]
    pub yellow: ::std::vec::Vec<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoScore {
    #[prost(sint32, tag = "1")]
    pub total: i32,
    #[prost(sint32, tag = "2")]
    pub red: i32,
    #[prost(sint32, tag = "3")]
    pub green: i32,
    #[prost(sint32, tag = "4")]
    pub white: i32,
    #[prost(sint32, tag = "5")]
    pub blue: i32,
    #[prost(sint32, tag = "6")]
    pub yellow: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGameMetadata {
    #[prost(string, tag = "1")]
    pub game_id: std::string::String,
    #[prost(string, tag = "2")]
    pub host_player_id: std::string::String,
    #[prost(string, tag = "3")]
    pub guest_player_id: std::string::String,
    #[prost(enumeration = "ProtoGameStatus", tag = "4")]
    pub status: i32,
    #[prost(uint64, tag = "5")]
    pub created_time_ms: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoGame {
    #[prost(message, repeated, tag = "1")]
    pub my_hand: ::std::vec::Vec<ProtoCard>,
    #[prost(message, optional, tag = "2")]
    pub my_plays: ::std::option::Option<ProtoPlayHistory>,
    #[prost(message, optional, tag = "3")]
    pub opponent_plays: ::std::option::Option<ProtoPlayHistory>,
    #[prost(message, optional, tag = "4")]
    pub discard_pile: ::std::option::Option<ProtoDiscardPile>,
    #[prost(uint32, tag = "5")]
    pub draw_pile_cards_remaining: u32,
    #[prost(enumeration = "ProtoGameStatus", tag = "6")]
    pub status: i32,
    #[prost(message, optional, tag = "7")]
    pub my_score: ::std::option::Option<ProtoScore>,
    #[prost(message, optional, tag = "8")]
    pub op_score: ::std::option::Option<ProtoScore>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoDiscardPile {
    #[prost(message, optional, tag = "1")]
    pub red: ::std::option::Option<ProtoDiscardPileSurface>,
    #[prost(message, optional, tag = "2")]
    pub green: ::std::option::Option<ProtoDiscardPileSurface>,
    #[prost(message, optional, tag = "3")]
    pub white: ::std::option::Option<ProtoDiscardPileSurface>,
    #[prost(message, optional, tag = "4")]
    pub blue: ::std::option::Option<ProtoDiscardPileSurface>,
    #[prost(message, optional, tag = "5")]
    pub yellow: ::std::option::Option<ProtoDiscardPileSurface>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoDiscardPileSurface {
    #[prost(uint32, tag = "1")]
    pub value: u32,
    #[prost(uint32, tag = "2")]
    pub remaining: u32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoColor {
    NoColor = 0,
    Red = 1,
    Green = 2,
    White = 3,
    Blue = 4,
    Yellow = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoDrawPile {
    NoDrawPile = 0,
    MainDraw = 1,
    DiscardDraw = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoPlayTarget {
    NoPlayTarget = 0,
    PlayerBoard = 1,
    Discard = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ProtoGameStatus {
    NoGameStatus = 0,
    YourTurn = 1,
    OpponentTurn = 2,
    EndWin = 3,
    EndLose = 4,
    EndDraw = 5,
    Unmatched = 6,
}
#[doc = r" Generated server implementations."]
pub mod proto_lost_cities_server {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    #[doc = "Generated trait containing gRPC methods that should be implemented for use with ProtoLostCitiesServer."]
    #[async_trait]
    pub trait ProtoLostCities: Send + Sync + 'static {
        async fn host_game(
            &self,
            request: tonic::Request<super::ProtoHostGameReq>,
        ) -> Result<tonic::Response<super::ProtoHostGameReply>, tonic::Status>;
        async fn join_game(
            &self,
            request: tonic::Request<super::ProtoJoinGameReq>,
        ) -> Result<tonic::Response<super::ProtoJoinGameReply>, tonic::Status>;
        async fn get_game_state(
            &self,
            request: tonic::Request<super::ProtoGetGameStateReq>,
        ) -> Result<tonic::Response<super::ProtoGetGameStateReply>, tonic::Status>;
        async fn play_card(
            &self,
            request: tonic::Request<super::ProtoPlayCardReq>,
        ) -> Result<tonic::Response<super::ProtoPlayCardReply>, tonic::Status>;
        async fn describe_game(
            &self,
            request: tonic::Request<super::ProtoDescribeGameReq>,
        ) -> Result<tonic::Response<super::ProtoDescribeGameReply>, tonic::Status>;
        async fn query_games(
            &self,
            request: tonic::Request<super::ProtoQueryGamesReq>,
        ) -> Result<tonic::Response<super::ProtoQueryGamesReply>, tonic::Status>;
        async fn get_matchable_games(
            &self,
            request: tonic::Request<super::ProtoGetMatchableGamesReq>,
        ) -> Result<tonic::Response<super::ProtoGetMatchableGamesReply>, tonic::Status>;
    }
    #[derive(Debug)]
    #[doc(hidden)]
    pub struct ProtoLostCitiesServer<T: ProtoLostCities> {
        inner: _Inner<T>,
    }
    struct _Inner<T>(Arc<T>, Option<tonic::Interceptor>);
    impl<T: ProtoLostCities> ProtoLostCitiesServer<T> {
        pub fn new(inner: T) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, None);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = Arc::new(inner);
            let inner = _Inner(inner, Some(interceptor.into()));
            Self { inner }
        }
    }
    impl<T: ProtoLostCities> Service<http::Request<HyperBody>> for ProtoLostCitiesServer<T> {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = Never;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(&mut self, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<HyperBody>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/proto_lost_cities.ProtoLostCities/HostGame" => {
                    struct HostGameSvc<T: ProtoLostCities>(pub Arc<T>);
                    impl<T: ProtoLostCities> tonic::server::UnaryService<super::ProtoHostGameReq> for HostGameSvc<T> {
                        type Response = super::ProtoHostGameReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProtoHostGameReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.host_game(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = HostGameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto_lost_cities.ProtoLostCities/JoinGame" => {
                    struct JoinGameSvc<T: ProtoLostCities>(pub Arc<T>);
                    impl<T: ProtoLostCities> tonic::server::UnaryService<super::ProtoJoinGameReq> for JoinGameSvc<T> {
                        type Response = super::ProtoJoinGameReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProtoJoinGameReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.join_game(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = JoinGameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto_lost_cities.ProtoLostCities/GetGameState" => {
                    struct GetGameStateSvc<T: ProtoLostCities>(pub Arc<T>);
                    impl<T: ProtoLostCities>
                        tonic::server::UnaryService<super::ProtoGetGameStateReq>
                        for GetGameStateSvc<T>
                    {
                        type Response = super::ProtoGetGameStateReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProtoGetGameStateReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_game_state(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetGameStateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto_lost_cities.ProtoLostCities/PlayCard" => {
                    struct PlayCardSvc<T: ProtoLostCities>(pub Arc<T>);
                    impl<T: ProtoLostCities> tonic::server::UnaryService<super::ProtoPlayCardReq> for PlayCardSvc<T> {
                        type Response = super::ProtoPlayCardReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProtoPlayCardReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.play_card(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = PlayCardSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto_lost_cities.ProtoLostCities/DescribeGame" => {
                    struct DescribeGameSvc<T: ProtoLostCities>(pub Arc<T>);
                    impl<T: ProtoLostCities>
                        tonic::server::UnaryService<super::ProtoDescribeGameReq>
                        for DescribeGameSvc<T>
                    {
                        type Response = super::ProtoDescribeGameReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProtoDescribeGameReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.describe_game(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = DescribeGameSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto_lost_cities.ProtoLostCities/QueryGames" => {
                    struct QueryGamesSvc<T: ProtoLostCities>(pub Arc<T>);
                    impl<T: ProtoLostCities> tonic::server::UnaryService<super::ProtoQueryGamesReq>
                        for QueryGamesSvc<T>
                    {
                        type Response = super::ProtoQueryGamesReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProtoQueryGamesReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.query_games(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = QueryGamesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/proto_lost_cities.ProtoLostCities/GetMatchableGames" => {
                    struct GetMatchableGamesSvc<T: ProtoLostCities>(pub Arc<T>);
                    impl<T: ProtoLostCities>
                        tonic::server::UnaryService<super::ProtoGetMatchableGamesReq>
                        for GetMatchableGamesSvc<T>
                    {
                        type Response = super::ProtoGetMatchableGamesReply;
                        type Future = BoxFuture<tonic::Response<Self::Response>, tonic::Status>;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ProtoGetMatchableGamesReq>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { inner.get_matchable_games(request).await };
                            Box::pin(fut)
                        }
                    }
                    let inner = self.inner.clone();
                    let fut = async move {
                        let interceptor = inner.1.clone();
                        let inner = inner.0;
                        let method = GetMatchableGamesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = if let Some(interceptor) = interceptor {
                            tonic::server::Grpc::with_interceptor(codec, interceptor)
                        } else {
                            tonic::server::Grpc::new(codec)
                        };
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => Box::pin(async move {
                    Ok(http::Response::builder()
                        .status(200)
                        .header("grpc-status", "12")
                        .body(tonic::body::BoxBody::empty())
                        .unwrap())
                }),
            }
        }
    }
    impl<T: ProtoLostCities> Clone for ProtoLostCitiesServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self { inner }
        }
    }
    impl<T: ProtoLostCities> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone(), self.1.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: ProtoLostCities> tonic::transport::NamedService for ProtoLostCitiesServer<T> {
        const NAME: &'static str = "proto_lost_cities.ProtoLostCities";
    }
}
