use common::{
    ecs::components::{EcsProtocol, Player},
    FromServerMessage,
};

use crate::server::Server;

pub fn execute(server: &mut Server, requester_id: u64) {
    if server.is_registered(requester_id) {
        let requester_info = server.registered_clients.clients.remove(&requester_id).expect(&format!("Can not unregister an non-existent participant with name '{requester_id}'"));

        // TODO: fix it not sending leave message
        // Notifies other participants about this removed participant
        let player_entity = server
            .ecs
            .world
            .query::<&Player>()
            .iter()
            .find(|(_, &player)| player.id == requester_id)
            .unwrap()
            .0;
        server.ecs.observed_world().despawn(player_entity).unwrap();

        FromServerMessage::EcsChanges(
            server
                .ecs
                .observer
                .drain_reliable()
                .collect::<Vec<EcsProtocol>>(),
        )
        .construct()
        .unwrap()
        .send_all(
            &server.handler,
            server.registered_clients.get_all_endpoints(),
        );

        println!(
            "Unregistered participant '{requester_id}' with ip {}",
            requester_info.id.addr
        );
    }
}
