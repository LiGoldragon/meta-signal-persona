//! Schema-derived MetaSignal contract for privileged Persona engine-manager
//! commands.
//!
//! This crate carries the meta policy surface for the top-level Persona
//! daemon: engine launch, retirement, component lifecycle orders, and manager
//! status queries. The ordinary manager-to-child lifecycle relation lives in
//! `signal-persona`.

#[rustfmt::skip]
pub mod schema;

pub use schema::lib::*;

pub type Operation = Input;
pub type OperationKind = InputRoute;
pub type Reply = Output;
pub type Query = MetaQuery;

impl Input {
    pub fn kind(&self) -> InputRoute {
        self.route()
    }
}
