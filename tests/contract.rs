use meta_signal_persona::{
    ActionAcceptance, ActionRejection, ActionRejectionReason, ByteViewable, EngineCatalogEntry,
    EngineCatalogScope, EnginePhase, EngineStatusReport, EngineStatusScope, LaunchAcceptance,
    LaunchRejection, LaunchRejectionReason, MetaQuery, Query, Response, Restorable,
    RetirementRejection, RetirementRejectionReason, Signal, Signalizable,
};
use signal_persona::{ComponentDesiredState, ComponentHealth, ComponentKind, ComponentStatus};

fn component_status() -> ComponentStatus {
    ComponentStatus {
        component_name: "router".into(),
        component_kind: ComponentKind::Router,
        component_desired_state: ComponentDesiredState::Running,
        component_health: ComponentHealth::Starting,
    }
}

fn queries() -> Vec<Query> {
    vec![
        Query::Launch("primary".into()),
        Query::Retire("engine-7".into()),
        Query::Start("router".into()),
        Query::Stop("mind".into()),
        Query::Query(MetaQuery::ComponentStatus("router".into())),
        Query::Query(MetaQuery::EngineStatus(EngineStatusScope::WholeEngine)),
        Query::Query(MetaQuery::Catalog(EngineCatalogScope::AllEngines)),
    ]
}

fn responses() -> Vec<Response> {
    vec![
        Response::Launched(LaunchAcceptance {
            engine_identifier: "engine-7".into(),
            engine_label: "primary".into(),
        }),
        Response::LaunchRejected(LaunchRejection {
            engine_label: "primary".into(),
            launch_rejection_reason: LaunchRejectionReason::EngineLimitReached,
        }),
        Response::Retired("engine-7".into()),
        Response::RetireRejected(RetirementRejection {
            engine_identifier: "engine-7".into(),
            retirement_rejection_reason: RetirementRejectionReason::EngineStillRunning,
        }),
        Response::Catalog(vec![EngineCatalogEntry {
            engine_identifier: "engine-7".into(),
            engine_label: "primary".into(),
            engine_phase: EnginePhase::Running,
        }]),
        Response::EngineStatus(EngineStatusReport {
            engine_generation: 4,
            engine_phase: EnginePhase::Running,
            component_status_vector: vec![component_status()],
        }),
        Response::ComponentStatus(component_status()),
        Response::ComponentMissing("trace".into()),
        Response::ActionAccepted(ActionAcceptance {
            component_name: "router".into(),
            component_desired_state: ComponentDesiredState::Running,
        }),
        Response::ActionRejected(ActionRejection {
            component_name: "mind".into(),
            action_rejection_reason: ActionRejectionReason::ComponentNotManaged,
        }),
    ]
}

#[test]
fn every_query_round_trips_through_received_bytes() {
    for query in queries() {
        let received =
            Signal::<Query>::from(query.signalize().expect("query archives").bytes().to_vec());
        assert_eq!(received.restore().expect("query restores"), query);
    }
}

#[test]
fn every_response_round_trips_through_received_bytes() {
    for response in responses() {
        let received = Signal::<Response>::from(
            response
                .signalize()
                .expect("response archives")
                .bytes()
                .to_vec(),
        );
        assert_eq!(received.restore().expect("response restores"), response);
    }
}

#[test]
fn malformed_archive_is_rejected() {
    assert!(Signal::<Query>::from(vec![1, 2, 3]).restore().is_err());
}

#[cfg(feature = "datom")]
#[test]
fn every_head_round_trips_as_datom_text() {
    use datom_codec::{Actualizing, Budget, Datomizable, Potential};
    use protos::{Protosizable, ReaderBudget, Textualizable};

    let budget = || Budget {
        remaining: 4096,
        reader: ReaderBudget { remaining: 4096 },
        depth: 0,
        maximum_depth: 1024,
    };

    for query in queries() {
        let text = query.clone().datomize(vec![]).protosize().textualize();
        let restored = Potential::<Query>::from(text.clone())
            .actualize(&mut budget())
            .unwrap_or_else(|fault| panic!("Query restores from {text}: {fault:?}"));
        assert_eq!(restored, query);
    }
    for response in responses() {
        let text = response.clone().datomize(vec![]).protosize().textualize();
        let restored = Potential::<Response>::from(text.clone())
            .actualize(&mut budget())
            .unwrap_or_else(|fault| panic!("Response restores from {text}: {fault:?}"));
        assert_eq!(restored, response);
    }
}

#[cfg(feature = "datom")]
#[test]
fn every_canonical_datom_line_actualizes_into_a_contract_head() {
    use datom_codec::{Actualizing, Budget, Potential};
    use protos::ReaderBudget;

    let budget = || Budget {
        remaining: 4096,
        reader: ReaderBudget { remaining: 4096 },
        depth: 0,
        maximum_depth: 1024,
    };

    let mut lines = 0;
    for line in include_str!("../examples/canonical.datom")
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with(';'))
    {
        lines += 1;
        let query = Potential::<Query>::from(line.to_string()).actualize(&mut budget());
        let response = Potential::<Response>::from(line.to_string()).actualize(&mut budget());
        assert!(
            query.is_ok() || response.is_ok(),
            "canonical line is neither a Query nor a Response: {line}\n  as Query: {query:?}\n  as Response: {response:?}"
        );
    }
    assert_eq!(lines, 17, "canonical file carries every contract head");
}
