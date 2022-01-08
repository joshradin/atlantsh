use atlantsh_bin::shared::ServerState;
use std::io::{Read, Write};

pub struct ServerInstance<In: Read, Out: Write> {
    state: ServerState,
}
