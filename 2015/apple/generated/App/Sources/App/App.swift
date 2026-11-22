import Observation
import Serde

func serializeArray<T, S: Serializer>(
    value: [T],
    serializer: S,
    serializeElement: (T, S) throws -> Void
) throws {
    try serializer.serialize_len(value: value.count)
    for item in value {
        try serializeElement(item, serializer)
    }
}

func deserializeArray<T, D: Deserializer>(
    deserializer: D,
    deserializeElement: (D) throws -> T
) throws -> [T] {
    let length = try deserializer.deserialize_len()
    var obj: [T] = []
    for _ in 0..<length {
        obj.append(try deserializeElement(deserializer))
    }
    return obj
}

indirect public enum Effect: Hashable, Equatable {
    case render(RenderOperation)

    public func serialize<S: Serializer>(serializer: S) throws {
        try serializer.increase_container_depth()
        switch self {
        case .render(let x):
            try serializer.serialize_variant_index(value: 0)
            try x.serialize(serializer: serializer)
        }
        try serializer.decrease_container_depth()
    }

    public func bincodeSerialize() throws -> [UInt8] {
        let serializer = BincodeSerializer.init();
        try self.serialize(serializer: serializer)
        return serializer.get_bytes()
    }

    public static func deserialize<D: Deserializer>(deserializer: D) throws -> Effect {
        let index = try deserializer.deserialize_variant_index()
        try deserializer.increase_container_depth()
        switch index {
        case 0:
            let x = try RenderOperation.deserialize(deserializer: deserializer)
            try deserializer.decrease_container_depth()
            return .render(x)
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index for Effect: \(index)")
        }
    }

    public static func bincodeDeserialize(input: [UInt8]) throws -> Effect {
        let deserializer = BincodeDeserializer.init(input: input);
        let obj = try deserialize(deserializer: deserializer)
        if deserializer.get_buffer_offset() < input.count {
            throw DeserializationError.invalidInput(issue: "Some input bytes were not read")
        }
        return obj
    }
}

/// How many times the shell is expected to resolve a request.
public enum OperationKind: Hashable, Sendable {
    /// Never — the core is not waiting for an answer.
    case notify
    /// Exactly once.
    case request
    /// Any number of times, until the shell stops sending.
    case stream
}

extension Effect {
    /// How many times the shell resolves a request carrying this effect,
    /// or `nil` when the operation leaves that to the call site.
    public var operationKind: OperationKind? {
        switch self {
        case .render: return .notify
        }
    }
}

/// Receives the items of a streaming effect.
///
/// Every item sent is one resolution of the request that started the stream.
public struct EffectSink<Item>: Sendable {
    private let _send: @Sendable (Item) -> Void

    public init(_ send: @escaping @Sendable (Item) -> Void) {
        self._send = send
    }

    /// Send one item back to the core.
    public func send(_ item: Item) {
        self._send(item)
    }
}

/// Serializing an effect output cannot fail for any value the core can
/// accept, so the dispatcher traps rather than making every handler
/// method throw.
private func serializeEffectOutput(_ body: (BincodeSerializer) throws -> Void) -> [UInt8] {
    let serializer = BincodeSerializer.init()
    try! body(serializer)
    return serializer.get_bytes()
}

/// Handles every effect `Effect` can carry.
///
/// `EffectDispatcher` resolves each request for you, so there is no
/// `resolve` to call at the wrong time or with the wrong bytes.
@available(macOS 10.15, iOS 13.0, tvOS 13.0, watchOS 6.0, *)
public protocol EffectHandler: Sendable {
    func render(_ operation: RenderOperation)
}

/// `Core` handles `Render` itself, so this does nothing; implement it
/// only if you drive `EffectDispatcher` yourself.
@available(macOS 10.15, iOS 13.0, tvOS 13.0, watchOS 6.0, *)
extension EffectHandler {
    public func render(_ operation: RenderOperation) {}
}

/// Routes each request to an `EffectHandler` and resolves it for you:
/// never for a notification, once for a request, once per item for a
/// stream.
@available(macOS 10.15, iOS 13.0, tvOS 13.0, watchOS 6.0, *)
public struct EffectDispatcher: Sendable {
    private let handler: any EffectHandler
    private let resolve: @Sendable (UInt32, [UInt8]) -> Void

    public init(handler: any EffectHandler, resolve: @escaping @Sendable (UInt32, [UInt8]) -> Void) {
        self.handler = handler
        self.resolve = resolve
    }

    public func dispatch(_ request: Request) {
        let handler = self.handler
        let resolve = self.resolve
        let id = request.id
        switch request.effect {
        case .render(let operation):
            handler.render(operation)
        }
    }
}

/// Which effect a request id names.
///
/// The raw value is the effect's position in `Effect`, which is what the
/// request id carries.
public enum EffectKind: UInt8, Hashable, Sendable {
    case render = 0
}

/// The parts of the id the core issues with every request.
///
/// Resolve with `rawValue`, exactly as it arrived — this is for logging,
/// tracing and assertions. The bit layout is Crux's, so read ids with this
/// rather than picking the integer apart by hand.
public struct RequestId: Hashable, Sendable {
    /// The id as it came from the core, and as it goes back.
    public let rawValue: UInt32

    public init(_ rawValue: UInt32) {
        self.rawValue = rawValue
    }

    /// Every notification shares one reserved id, because the core is not
    /// waiting on any of them.
    public var isNotification: Bool {
        self.rawValue == 0
    }

    /// Which effect this request carries — `nil` for a notification, whose
    /// id names none, and for an id naming an effect this generated code
    /// does not know.
    public var effectKind: EffectKind? {
        if self.isNotification { return nil }
        return EffectKind(rawValue: UInt8(truncatingIfNeeded: self.rawValue >> 24))
    }

    /// How many times the core expects this request to be resolved.
    public var operationKind: OperationKind {
        if self.isNotification { return .notify }
        return (self.rawValue & 0x800000) == 0 ? .request : .stream
    }

    /// The ascending number the request was issued with. Zero for a
    /// notification, the one id that is not sequenced.
    public var sequence: UInt32 {
        self.rawValue & 0x7fffff
    }
}

/// The core's FFI surface, as bytes. Implement it around the `CoreFfi` your
/// Rust crate exports, or with a fake for previews and tests.
public protocol CoreBridge: Sendable {
    func update(_ event: [UInt8]) -> [UInt8]
    func resolve(_ id: UInt32, _ output: [UInt8]) -> [UInt8]
    func view() -> [UInt8]
}

/// The shell's side of the core: it serializes events, dispatches the
/// requests that come back, and resolves them through the bridge.
///
/// `Render` is handled here, so an `EffectHandler` need not implement it.
///
/// `view` is observable, so a SwiftUI view that reads it is invalidated
/// whenever the core renders.
@available(macOS 14.0, iOS 17.0, tvOS 17.0, watchOS 10.0, *)
@Observable
@MainActor
public final class Core {
    /// The view model as of the last `Render`.
    public private(set) var view: ViewModel
    @ObservationIgnored private let bridge: any CoreBridge
    @ObservationIgnored private var dispatcher: EffectDispatcher!

    public init(bridge: any CoreBridge, handler: any EffectHandler) {
        self.bridge = bridge
        self.view = try! ViewModel.bincodeDeserialize(input: bridge.view())
        self.dispatcher = EffectDispatcher(handler: handler) { [weak self] id, bytes in
            Task { @MainActor in
                guard let self else { return }
                self.process(bytes: self.bridge.resolve(id, bytes))
            }
        }
    }

    /// Send an event to the core and run the loop it starts.
    public func update(_ event: Event) {
        process(bytes: bridge.update(try! event.bincodeSerialize()))
    }

    /// Serialized requests, as returned by the bridge or pushed by middleware.
    public func process(bytes: [UInt8]) {
        if bytes.isEmpty { return }
        process(try! Requests.bincodeDeserialize(input: bytes).value)
    }

    public func process(_ requests: [Request]) {
        for request in requests {
            if case .render = request.effect {
                view = try! ViewModel.bincodeDeserialize(input: bridge.view())
                continue
            }
            dispatcher.dispatch(request)
        }
    }
}

indirect public enum Event: Hashable, Equatable {
    case increment
    case decrement
    case reset

    public func serialize<S: Serializer>(serializer: S) throws {
        try serializer.increase_container_depth()
        switch self {
        case .increment:
            try serializer.serialize_variant_index(value: 0)
        case .decrement:
            try serializer.serialize_variant_index(value: 1)
        case .reset:
            try serializer.serialize_variant_index(value: 2)
        }
        try serializer.decrease_container_depth()
    }

    public func bincodeSerialize() throws -> [UInt8] {
        let serializer = BincodeSerializer.init();
        try self.serialize(serializer: serializer)
        return serializer.get_bytes()
    }

    public static func deserialize<D: Deserializer>(deserializer: D) throws -> Event {
        let index = try deserializer.deserialize_variant_index()
        try deserializer.increase_container_depth()
        switch index {
        case 0:
            try deserializer.decrease_container_depth()
            return .increment
        case 1:
            try deserializer.decrease_container_depth()
            return .decrement
        case 2:
            try deserializer.decrease_container_depth()
            return .reset
        default: throw DeserializationError.invalidInput(issue: "Unknown variant index for Event: \(index)")
        }
    }

    public static func bincodeDeserialize(input: [UInt8]) throws -> Event {
        let deserializer = BincodeDeserializer.init(input: input);
        let obj = try deserialize(deserializer: deserializer)
        if deserializer.get_buffer_offset() < input.count {
            throw DeserializationError.invalidInput(issue: "Some input bytes were not read")
        }
        return obj
    }
}

/// The single operation `Render` implements.
public struct RenderOperation: Hashable, Equatable {
    public init() {
    }

    public func serialize<S: Serializer>(serializer: S) throws {
        try serializer.increase_container_depth()
        try serializer.decrease_container_depth()
    }

    public func bincodeSerialize() throws -> [UInt8] {
        let serializer = BincodeSerializer.init();
        try self.serialize(serializer: serializer)
        return serializer.get_bytes()
    }

    public static func deserialize<D: Deserializer>(deserializer: D) throws -> RenderOperation {
        try deserializer.increase_container_depth()
        try deserializer.decrease_container_depth()
        return RenderOperation()
    }

    public static func bincodeDeserialize(input: [UInt8]) throws -> RenderOperation {
        let deserializer = BincodeDeserializer.init(input: input);
        let obj = try deserialize(deserializer: deserializer)
        if deserializer.get_buffer_offset() < input.count {
            throw DeserializationError.invalidInput(issue: "Some input bytes were not read")
        }
        return obj
    }
}

/// Request for a side-effect passed from the Core to the Shell.
/// 
/// The `EffectId` links the `Request` with the corresponding call to [`Core::resolve`] to pass the data back
/// to the [`App::update`] function (wrapped in the event provided to the capability originating the effect).
public struct Request: Hashable, Equatable {
    public var id: UInt32
    public var effect: Effect

    public init(id: UInt32, effect: Effect) {
        self.id = id
        self.effect = effect
    }

    public func serialize<S: Serializer>(serializer: S) throws {
        try serializer.increase_container_depth()
        try serializer.serialize_u32(value: self.id)
        try self.effect.serialize(serializer: serializer)
        try serializer.decrease_container_depth()
    }

    public func bincodeSerialize() throws -> [UInt8] {
        let serializer = BincodeSerializer.init();
        try self.serialize(serializer: serializer)
        return serializer.get_bytes()
    }

    public static func deserialize<D: Deserializer>(deserializer: D) throws -> Request {
        try deserializer.increase_container_depth()
        let id = try deserializer.deserialize_u32()
        let effect = try Effect.deserialize(deserializer: deserializer)
        try deserializer.decrease_container_depth()
        return Request(id: id, effect: effect)
    }

    public static func bincodeDeserialize(input: [UInt8]) throws -> Request {
        let deserializer = BincodeDeserializer.init(input: input);
        let obj = try deserialize(deserializer: deserializer)
        if deserializer.get_buffer_offset() < input.count {
            throw DeserializationError.invalidInput(issue: "Some input bytes were not read")
        }
        return obj
    }
}

/// A batch of effect requests from the Core to the Shell, as serialised by
/// [`Bridge::update`] and [`Bridge::resolve`].
/// 
/// The wire format is identical to `Vec<Request<Eff>>` (the newtype is
/// `serde(transparent)`), so existing shell code that already deserialises
/// a `Vec<Request>` remains binary-compatible.
/// 
/// Registering this type with the type-generation system causes the code
/// generators to emit a `Requests` type (with a `value` field containing the
/// list) together with a top-level `bincodeDeserialize` / `BincodeDeserialize`
/// helper, replacing the hand-written extension files that were previously
/// appended by `add_extensions()`.
public struct Requests: Hashable, Equatable {
    public var value: [Request]

    public init(value: [Request]) {
        self.value = value
    }

    public func serialize<S: Serializer>(serializer: S) throws {
        try serializer.increase_container_depth()
        try serializeArray(value: self.value, serializer: serializer) { item, serializer in
            try item.serialize(serializer: serializer)
        }
        try serializer.decrease_container_depth()
    }

    public func bincodeSerialize() throws -> [UInt8] {
        let serializer = BincodeSerializer.init();
        try self.serialize(serializer: serializer)
        return serializer.get_bytes()
    }

    public static func deserialize<D: Deserializer>(deserializer: D) throws -> Requests {
        try deserializer.increase_container_depth()
        let value = try deserializeArray(deserializer: deserializer) { deserializer in
            try Request.deserialize(deserializer: deserializer)
        }
        try deserializer.decrease_container_depth()
        return Requests(value: value)
    }

    public static func bincodeDeserialize(input: [UInt8]) throws -> Requests {
        let deserializer = BincodeDeserializer.init(input: input);
        let obj = try deserialize(deserializer: deserializer)
        if deserializer.get_buffer_offset() < input.count {
            throw DeserializationError.invalidInput(issue: "Some input bytes were not read")
        }
        return obj
    }
}

public struct ViewModel: Hashable, Equatable {
    public var count: String

    public init(count: String) {
        self.count = count
    }

    public func serialize<S: Serializer>(serializer: S) throws {
        try serializer.increase_container_depth()
        try serializer.serialize_str(value: self.count)
        try serializer.decrease_container_depth()
    }

    public func bincodeSerialize() throws -> [UInt8] {
        let serializer = BincodeSerializer.init();
        try self.serialize(serializer: serializer)
        return serializer.get_bytes()
    }

    public static func deserialize<D: Deserializer>(deserializer: D) throws -> ViewModel {
        try deserializer.increase_container_depth()
        let count = try deserializer.deserialize_str()
        try deserializer.decrease_container_depth()
        return ViewModel(count: count)
    }

    public static func bincodeDeserialize(input: [UInt8]) throws -> ViewModel {
        let deserializer = BincodeDeserializer.init(input: input);
        let obj = try deserialize(deserializer: deserializer)
        if deserializer.get_buffer_offset() < input.count {
            throw DeserializationError.invalidInput(issue: "Some input bytes were not read")
        }
        return obj
    }
}
