use meta_signal_persona::{
    ActionAcceptance, ActionRejection, ActionRejectionReason, ComponentDesiredState, ComponentName,
    ComponentShutdown, ComponentStartup, EngineCatalog, EngineCatalogEntry, EngineCatalogScope,
    EngineIdentifier, EngineLabel, EngineLaunch, EnginePhase, EngineStatusScope, Frame, FrameBody,
    Operation, OperationKind, Query, Reply, RetirementRejection, RetirementRejectionReason,
    short_header,
};
#[cfg(feature = "nota-text")]
use meta_signal_persona::{
    ComponentHealth, ComponentKind, EngineGeneration, EngineStatusReport, LifecycleComponentStatus,
};
#[cfg(feature = "nota-text")]
use nota::{NotaEncode, NotaSource};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, SubReply,
};

fn exchange() -> ExchangeIdentifier {
    ExchangeIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Connector,
        LaneSequence::first(),
    )
}

fn completed_reply(payload: Reply) -> FrameReply<Reply> {
    FrameReply::committed(NonEmpty::single(SubReply::Ok(payload)))
}

fn engine_identifier(label: &str) -> EngineIdentifier {
    EngineIdentifier::new(label)
}

fn router_name() -> ComponentName {
    ComponentName::new("persona-router")
}

#[cfg(feature = "nota-text")]
fn router_status() -> LifecycleComponentStatus {
    LifecycleComponentStatus {
        component_name: router_name(),
        component_kind: ComponentKind::Router,
        component_desired_state: ComponentDesiredState::Running,
        component_health: ComponentHealth::Running,
    }
}

fn launch_operation() -> Operation {
    Operation::Launch(EngineLaunch::new(EngineLabel::new("research")).into())
}

fn round_trip_operation(operation: Operation) -> Operation {
    let frame = Frame::new(FrameBody::Request {
        exchange: exchange(),
        request: operation.clone().into_request(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode operation");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode operation");

    match decoded.into_body() {
        FrameBody::Request { request, .. } => request.payloads().head().clone(),
        other => panic!("expected request, got {other:?}"),
    }
}

fn round_trip_reply(reply: Reply) -> Reply {
    let frame = Frame::new(FrameBody::Reply {
        exchange: exchange(),
        reply: completed_reply(reply.clone()),
    });
    let bytes = frame.encode_length_prefixed().expect("encode reply");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode reply");

    match decoded.into_body() {
        FrameBody::Reply { reply, .. } => match reply {
            FrameReply::Accepted { per_operation, .. } => match per_operation.into_head() {
                SubReply::Ok(payload) => payload,
                other => panic!("expected accepted reply payload, got {other:?}"),
            },
            other => panic!("expected accepted reply, got {other:?}"),
        },
        other => panic!("expected reply, got {other:?}"),
    }
}

#[test]
fn meta_operations_round_trip_through_length_prefixed_frames() {
    assert_eq!(round_trip_operation(launch_operation()), launch_operation());

    let catalog = Operation::Query(Query::Catalog(EngineCatalogScope::AllEngines).into());
    assert_eq!(round_trip_operation(catalog.clone()), catalog);

    let retire = Operation::Retire(engine_identifier("research").into());
    assert_eq!(round_trip_operation(retire.clone()), retire);

    let start = Operation::Start(ComponentStartup::new(router_name()).into());
    assert_eq!(round_trip_operation(start.clone()), start);

    let stop = Operation::Stop(ComponentShutdown::new(router_name()).into());
    assert_eq!(round_trip_operation(stop.clone()), stop);
}

#[test]
fn meta_replies_round_trip_through_length_prefixed_frames() {
    let catalog = Reply::Catalog(
        EngineCatalog::new(vec![EngineCatalogEntry {
            engine_identifier: engine_identifier("default"),
            engine_label: EngineLabel::new("default"),
            engine_phase: EnginePhase::Running,
        }])
        .into(),
    );
    assert_eq!(round_trip_reply(catalog.clone()), catalog);

    let blocked = Reply::RetireRejected(
        RetirementRejection {
            engine_identifier: engine_identifier("default"),
            retirement_rejection_reason: RetirementRejectionReason::EngineStillRunning,
        }
        .into(),
    );
    assert_eq!(round_trip_reply(blocked.clone()), blocked);
}

#[test]
fn generated_short_headers_are_contract_local_and_distinct() {
    let headers = [
        short_header::INPUT_LAUNCH,
        short_header::INPUT_QUERY,
        short_header::INPUT_RETIRE,
        short_header::INPUT_START,
        short_header::INPUT_STOP,
        short_header::OUTPUT_LAUNCHED,
        short_header::OUTPUT_LAUNCH_REJECTED,
        short_header::OUTPUT_CATALOG,
        short_header::OUTPUT_ENGINE_STATUS,
        short_header::OUTPUT_COMPONENT_STATUS,
        short_header::OUTPUT_COMPONENT_MISSING,
        short_header::OUTPUT_RETIRED,
        short_header::OUTPUT_RETIRE_REJECTED,
        short_header::OUTPUT_ACTION_ACCEPTED,
        short_header::OUTPUT_ACTION_REJECTED,
    ];
    for (outer_index, outer) in headers.iter().enumerate() {
        for (inner_index, inner) in headers.iter().enumerate() {
            if outer_index != inner_index {
                assert_ne!(outer, inner, "short headers must be distinct");
            }
        }
    }
}

#[cfg(feature = "nota-text")]
#[test]
fn meta_text_shape_stays_canonical() {
    let text = launch_operation().to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<Operation>()
        .expect("decode operation");
    assert_eq!(recovered, launch_operation());
    assert_eq!(text, "(Launch research)");

    let reply = Reply::EngineStatus(
        EngineStatusReport {
            engine_generation: EngineGeneration::new(1),
            engine_phase: EnginePhase::Running,
            lifecycle_component_status_vector: vec![router_status()],
        }
        .into(),
    );
    let text = reply.to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<Reply>()
        .expect("decode reply");
    assert_eq!(recovered, reply);
    assert_eq!(
        text,
        "(EngineStatus (1 Running [(persona-router Router Running Running)]))"
    );
}

#[test]
fn operation_kind_is_generated_by_schema() {
    let cases = [
        (launch_operation(), OperationKind::Launch),
        (
            Operation::Query(Query::EngineStatus(EngineStatusScope::WholeEngine).into()),
            OperationKind::Query,
        ),
        (
            Operation::Start(ComponentStartup::new(router_name()).into()),
            OperationKind::Start,
        ),
        (
            Operation::Stop(ComponentShutdown::new(router_name()).into()),
            OperationKind::Stop,
        ),
    ];

    for (operation, expected_kind) in cases {
        assert_eq!(operation.kind(), expected_kind);
        assert_eq!(operation.route(), expected_kind);
    }
}

#[test]
fn component_action_replies_stay_meta_policy_only() {
    let accepted = Reply::ActionAccepted(
        ActionAcceptance {
            component_name: router_name(),
            component_desired_state: ComponentDesiredState::Running,
        }
        .into(),
    );
    assert_eq!(round_trip_reply(accepted.clone()), accepted);

    let rejected = Reply::ActionRejected(
        ActionRejection {
            component_name: router_name(),
            action_rejection_reason: ActionRejectionReason::ComponentNotManaged,
        }
        .into(),
    );
    assert_eq!(round_trip_reply(rejected.clone()), rejected);
}
