use tonic::{Request, Response, Status};
use tonic::transport::Server;
use wire_types::proto_lost_cities::proto_lost_cities_server::{
    ProtoLostCities,
    ProtoLostCitiesServer
};
use wire_types::proto_lost_cities::{
    ProtoHostGameReq,
    ProtoHostGameReply,
    ProtoJoinGameReq,
    ProtoJoinGameReply,
    ProtoGetGameStateReq,
    ProtoGetGameStateReply,
    ProtoPlayCardReq,
    ProtoPlayCardReply,
};

#[derive(Default)]
pub struct MyServer {}

#[tonic::async_trait]
impl ProtoLostCities for MyServer {

    async fn host_game(&self, request: Request<ProtoHostGameReq>) -> Result<Response<ProtoHostGameReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let reply = ProtoHostGameReply {
            game_id: "".to_string()
        };

        Ok(Response::new(reply))
    }

    async fn join_game(&self, request: Request<ProtoJoinGameReq>) -> Result<Response<ProtoJoinGameReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let reply = ProtoJoinGameReply {};

        Ok(Response::new(reply))
    }

    async fn get_game_state(&self, request: Request<ProtoGetGameStateReq>) -> Result<Response<ProtoGetGameStateReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let reply = ProtoGetGameStateReply {
            game: None,
            opponent_player_id: "".to_string()
        };

        Ok(Response::new(reply))
    }

    async fn play_card(&self, request: Request<ProtoPlayCardReq>) -> Result<Response<ProtoPlayCardReply>, Status> {
        let inner = request.into_inner();
        println!("Rcv: {:?}", inner);

        let reply = ProtoPlayCardReply {};

        Ok(Response::new(reply))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse().unwrap();
    let greeter = MyServer::default();

    println!("GreeterServer listening on {}", addr);

    Server::builder()
        .add_service(ProtoLostCitiesServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}
