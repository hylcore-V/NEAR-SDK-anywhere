use super::{KnownProducerView, PeerInfoView};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct NetworkInfoView {
    pub peer_max_count: u32,
    pub num_connected_peers: usize,
    pub connected_peers: Vec<PeerInfoView>,
    pub known_producers: Vec<KnownProducerView>,
}
