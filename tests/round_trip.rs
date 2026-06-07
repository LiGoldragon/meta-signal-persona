use nota_next::{NotaEncode, NotaSource};
use owner_signal_persona::{
    ActionAcceptance, ActionRejection, ActionRejectionReason, ComponentDesiredState,
    ComponentHealth, ComponentName, ComponentShutdown, ComponentStartup, ComponentStatus,
    EffectEmitted, EffectOutcome, EngineCatalog, EngineCatalogEntry, EngineCatalogScope,
    EngineGeneration, EngineLabel, EngineLaunch, EnginePhase, EngineStatus, EngineStatusScope,
    Event, Frame, FrameBody, Operation, OperationKind, OperationReceived, Query, Reply,
    RetirementRejection, RetirementRejectionReason,
};
use signal_frame::{
    ExchangeIdentifier, ExchangeLane, LaneSequence, NonEmpty, Reply as FrameReply, RequestPayload,
    SessionEpoch, StreamEventIdentifier, SubReply, SubscriptionTokenInner,
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

fn event_identifier() -> StreamEventIdentifier {
    StreamEventIdentifier::new(
        SessionEpoch::new(1),
        ExchangeLane::Acceptor,
        LaneSequence::first(),
    )
}

fn engine_identifier(label: &str) -> signal_persona_origin::EngineIdentifier {
    signal_persona_origin::EngineIdentifier::new(label)
}

fn router_name() -> ComponentName {
    ComponentName::new("persona-router")
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

fn round_trip_event(event: Event) -> Event {
    let frame = Frame::new(FrameBody::SubscriptionEvent {
        event_identifier: event_identifier(),
        token: SubscriptionTokenInner::new(11),
        event: event.clone(),
    });
    let bytes = frame.encode_length_prefixed().expect("encode event");
    let decoded = Frame::decode_length_prefixed(&bytes).expect("decode event");

    match decoded.into_body() {
        FrameBody::SubscriptionEvent { event, .. } => event,
        other => panic!("expected subscription event, got {other:?}"),
    }
}

#[test]
fn owner_operations_round_trip_through_length_prefixed_frames() {
    let launch = Operation::Launch(EngineLaunch {
        label: EngineLabel::new("research"),
    });
    assert_eq!(round_trip_operation(launch.clone()), launch);

    let catalog = Operation::Query(Query::Catalog(EngineCatalogScope::AllEngines));
    assert_eq!(round_trip_operation(catalog.clone()), catalog);

    let retire = Operation::Retire(engine_identifier("research"));
    assert_eq!(round_trip_operation(retire.clone()), retire);
}

#[test]
fn owner_replies_round_trip_through_length_prefixed_frames() {
    let catalog = Reply::Catalog(EngineCatalog {
        engines: vec![EngineCatalogEntry {
            engine: engine_identifier("default"),
            label: EngineLabel::new("default"),
            phase: EnginePhase::Running,
        }],
    });
    assert_eq!(round_trip_reply(catalog.clone()), catalog);

    let blocked = Reply::RetireRejected(RetirementRejection {
        engine: engine_identifier("default"),
        reason: RetirementRejectionReason::EngineStillRunning,
    });
    assert_eq!(round_trip_reply(blocked.clone()), blocked);
}

#[test]
fn owner_events_round_trip_through_length_prefixed_frames() {
    let operation = Event::OperationReceived(OperationReceived {
        operation: OperationKind::Launch,
    });
    assert_eq!(round_trip_event(operation.clone()), operation);

    let effect = Event::EffectEmitted(EffectEmitted {
        operation: OperationKind::Launch,
        outcome: EffectOutcome::Launched,
    });
    assert_eq!(round_trip_event(effect.clone()), effect);
}

#[test]
fn owner_text_shape_stays_canonical() {
    let request = Operation::Launch(EngineLaunch {
        label: EngineLabel::new("research"),
    });
    let text = request.to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<Operation>()
        .expect("decode operation");
    assert_eq!(recovered, request);
    assert_eq!(text, "(Launch ([research]))");

    let reply = Reply::EngineStatus(EngineStatus {
        generation: EngineGeneration::new(1),
        phase: EnginePhase::Running,
        components: vec![ComponentStatus {
            name: router_name(),
            kind: owner_signal_persona::ComponentKind::Router,
            desired_state: ComponentDesiredState::Running,
            health: ComponentHealth::Running,
        }],
    });
    let text = reply.to_nota();
    let recovered = NotaSource::new(&text)
        .parse::<Reply>()
        .expect("decode reply");
    assert_eq!(recovered, reply);
    assert_eq!(
        text,
        "(EngineStatus (1 Running [([persona-router] Router Running Running)]))"
    );
}

#[test]
fn operation_kind_is_generated_by_macro() {
    let cases = [
        (
            Operation::Launch(EngineLaunch {
                label: EngineLabel::new("research"),
            }),
            OperationKind::Launch,
        ),
        (
            Operation::Query(Query::EngineStatus(EngineStatusScope::WholeEngine)),
            OperationKind::Query,
        ),
        (
            Operation::Start(ComponentStartup {
                component: router_name(),
            }),
            OperationKind::Start,
        ),
        (
            Operation::Stop(ComponentShutdown {
                component: router_name(),
            }),
            OperationKind::Stop,
        ),
    ];

    for (operation, expected_kind) in cases {
        assert_eq!(operation.kind(), expected_kind);
    }
}

#[test]
fn component_action_replies_stay_owner_only() {
    let accepted = Reply::ActionAccepted(ActionAcceptance {
        component: router_name(),
        desired_state: ComponentDesiredState::Running,
    });
    assert_eq!(round_trip_reply(accepted.clone()), accepted);

    let rejected = Reply::ActionRejected(ActionRejection {
        component: router_name(),
        reason: ActionRejectionReason::ComponentNotManaged,
    });
    assert_eq!(round_trip_reply(rejected.clone()), rejected);
}
