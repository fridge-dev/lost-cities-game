use std::net::SocketAddr;
use tonic::transport::Server;
use wire_types::proto_lost_cities::proto_lost_cities_server::ProtoLostCitiesServer;
use server::server_impl::LostCitiesBackendServer;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr: SocketAddr = "[::1]:50051".parse().expect("This should never happen. It's a valid IP address, dammit.");
    let server_impl = LostCitiesBackendServer::new();

    Server::builder()
        .add_service(ProtoLostCitiesServer::new(server_impl))
        .serve(addr)
        .await?;

    Ok(())
}
