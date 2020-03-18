// =======================================
// API Request and Reply messages
// =======================================

// Convention: ALL messages should have prefix "Proto" so in the rust src, it's easy
// to understand which types are generated.

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoHostGameReq {
    #[prost(string, tag = "1")]
    pub player_id: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtoHostGameReply {
    #[prost(string, tag = "1")]
    pub game_id: std::string::String,
}
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
    #[prost(sint32, tag = "7")]
    pub my_score: i32,
    #[prost(sint32, tag = "8")]
    pub op_score: i32,
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
}
#[doc = r" Generated client implementations."]
pub mod proto_lost_cities_client {
    #![allow(unused_variables, dead_code, missing_docs)]
    use tonic::codegen::*;
    pub struct ProtoLostCitiesClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl ProtoLostCitiesClient<tonic::transport::Channel> {
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
    impl<T> ProtoLostCitiesClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::ResponseBody: Body + HttpBody + Send + 'static,
        T::Error: Into<StdError>,
        <T::ResponseBody as HttpBody>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor(inner: T, interceptor: impl Into<tonic::Interceptor>) -> Self {
            let inner = tonic::client::Grpc::with_interceptor(inner, interceptor);
            Self { inner }
        }
        pub async fn host_game(
            &mut self,
            request: impl tonic::IntoRequest<super::ProtoHostGameReq>,
        ) -> Result<tonic::Response<super::ProtoHostGameReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto_lost_cities.ProtoLostCities/HostGame");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn join_game(
            &mut self,
            request: impl tonic::IntoRequest<super::ProtoJoinGameReq>,
        ) -> Result<tonic::Response<super::ProtoJoinGameReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto_lost_cities.ProtoLostCities/JoinGame");
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn get_game_state(
            &mut self,
            request: impl tonic::IntoRequest<super::ProtoGetGameStateReq>,
        ) -> Result<tonic::Response<super::ProtoGetGameStateReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/proto_lost_cities.ProtoLostCities/GetGameState",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        pub async fn play_card(
            &mut self,
            request: impl tonic::IntoRequest<super::ProtoPlayCardReq>,
        ) -> Result<tonic::Response<super::ProtoPlayCardReply>, tonic::Status> {
            self.inner.ready().await.map_err(|e| {
                tonic::Status::new(
                    tonic::Code::Unknown,
                    format!("Service was not ready: {}", e.into()),
                )
            })?;
            let codec = tonic::codec::ProstCodec::default();
            let path =
                http::uri::PathAndQuery::from_static("/proto_lost_cities.ProtoLostCities/PlayCard");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
    impl<T: Clone> Clone for ProtoLostCitiesClient<T> {
        fn clone(&self) -> Self {
            Self {
                inner: self.inner.clone(),
            }
        }
    }
}
