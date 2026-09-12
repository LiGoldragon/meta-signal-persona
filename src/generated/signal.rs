#![allow(dead_code, non_camel_case_types, non_snake_case)]
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum EngineCatalogScope {
    AllEngines,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum EnginePhase {
    Stopped,
    Degraded,
    Starting,
    Running,
    Draining,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EngineStatusReport {
    pub engine_generation: EngineGeneration,
    pub engine_phase: EnginePhase,
    pub component_status_vector: std::vec::Vec<signal_persona::ComponentStatus>,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum LaunchRejectionReason {
    LaunchPlanRejected,
    EngineLimitReached,
    EngineLabelAlreadyExists,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct EngineCatalogEntry {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub engine_label: EngineLabel,
    pub engine_phase: EnginePhase,
}
#[rustfmt::skip]
pub type EngineCatalog = std::vec::Vec<EngineCatalogEntry>;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ActionRejection {
    pub component_name: signal_persona::ComponentName,
    pub action_rejection_reason: ActionRejectionReason,
}
#[rustfmt::skip]
pub type EngineLaunch = EngineLabel;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct RetirementRejection {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub retirement_rejection_reason: RetirementRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LaunchRejection {
    pub engine_label: EngineLabel,
    pub launch_rejection_reason: LaunchRejectionReason,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum RetirementRejectionReason {
    EngineNotFound,
    EngineHasLiveRoutes,
    EngineStillRunning,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum ActionRejectionReason {
    ComponentAlreadyInDesiredState,
    ComponentNotManaged,
}
#[rustfmt::skip]
pub type ComponentStartup = signal_persona::ComponentName;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum EngineStatusScope {
    WholeEngine,
}
#[rustfmt::skip]
pub type EngineLabel = String;
#[rustfmt::skip]
pub type EngineGeneration = i64;
#[rustfmt::skip]
pub type ComponentShutdown = signal_persona::ComponentName;
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum MetaQuery {
    ComponentStatus(signal_persona::ComponentName),
    EngineStatus(EngineStatusScope),
    Catalog(EngineCatalogScope),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct ActionAcceptance {
    pub component_name: signal_persona::ComponentName,
    pub component_desired_state: signal_persona::ComponentDesiredState,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub struct LaunchAcceptance {
    pub engine_identifier: signal_persona::EngineIdentifier,
    pub engine_label: EngineLabel,
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Query {
    Retire(signal_persona::EngineIdentifier),
    Start(ComponentStartup),
    Launch(EngineLaunch),
    Query(MetaQuery),
    Stop(ComponentShutdown),
}
#[rustfmt::skip]
#[derive(rkyv::Archive, rkyv::Serialize, rkyv::Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "datom", derive(datom_codec::Datomizable, datom_codec::Composing))]
pub enum Response {
    ActionAccepted(ActionAcceptance),
    RetireRejected(RetirementRejection),
    Catalog(EngineCatalog),
    ComponentMissing(signal_persona::ComponentName),
    EngineStatus(EngineStatusReport),
    Launched(LaunchAcceptance),
    ActionRejected(ActionRejection),
    LaunchRejected(LaunchRejection),
    ComponentStatus(signal_persona::ComponentStatus),
    Retired(signal_persona::EngineIdentifier),
}
