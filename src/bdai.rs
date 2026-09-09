// BD AI: pre-seed the recent-sessions list with the BDS machines staff should reach.
//
// The list is baked in at compile time from BDAI_DEFAULT_PEERS (set by the build workflow
// from a repository variable, never committed): "id|alias|platform|hostname;id|alias|...".
// Seeding runs once per list value, only creates entries that do not exist yet, and never
// stores a password — staff still authenticate against the target's permanent password.
use hbb_common::{
    config::{LocalConfig, PeerConfig},
    log,
};

const SEEDED_KEY: &str = "bdai-seeded-peers";

pub fn seed_default_peers() {
    let Some(spec) = option_env!("BDAI_DEFAULT_PEERS") else {
        return;
    };
    let spec = spec.trim();
    if spec.is_empty() || LocalConfig::get_option(SEEDED_KEY) == spec {
        return;
    }
    let mut first_id: Option<String> = None;
    for entry in spec.split(';') {
        let mut parts = entry.split('|').map(str::trim);
        let id = parts.next().unwrap_or_default();
        if id.is_empty() {
            continue;
        }
        first_id.get_or_insert_with(|| id.to_owned());
        if PeerConfig::exists(id) {
            continue;
        }
        let alias = parts.next().unwrap_or_default();
        let platform = parts.next().filter(|p| !p.is_empty()).unwrap_or("Windows");
        let hostname = parts.next().unwrap_or_default();
        let mut peer = PeerConfig::default();
        // An empty platform makes PeerConfig::batch_peers delete the entry on load.
        peer.info.platform = platform.to_owned();
        peer.info.hostname = hostname.to_owned();
        if !alias.is_empty() {
            peer.options.insert("alias".to_owned(), alias.to_owned());
        }
        peer.store(id);
        log::info!("BD AI: seeded default peer {}", id);
    }
    // Pre-fill the connect box with the first BDS machine, unless the user has connected somewhere already.
    if let Some(id) = first_id {
        if LocalConfig::get_remote_id().is_empty() {
            LocalConfig::set_remote_id(&id);
        }
    }
    LocalConfig::set_option(SEEDED_KEY.to_owned(), spec.to_owned());
}
