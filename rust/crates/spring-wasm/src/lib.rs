//! Shared, owned semantic façade for native and Wasm Spring modules.
//!
//! This crate intentionally contains no host-OS bindings. Native and Wasm
//! transports implement [`CalloutBackend`], while module code deals only in
//! owned values and stable error categories.

use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Environment {
    RulesSynced,
    RulesUnsynced,
    GaiaSynced,
    GaiaUnsynced,
    Ui,
}

impl Environment {
    pub const fn is_synced(self) -> bool {
        matches!(self, Self::RulesSynced | Self::GaiaSynced)
    }

    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RulesSynced => "rules-synced",
            Self::RulesUnsynced => "rules-unsynced",
            Self::GaiaSynced => "gaia-synced",
            Self::GaiaUnsynced => "gaia-unsynced",
            Self::Ui => "ui",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCategory {
    InvalidArgument,
    Unavailable,
    InvalidHandle,
    ResourceLimit,
    ReentryDenied,
    GuestFault,
    VersionMismatch,
    Internal,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ApiError {
    pub code: i32,
    pub category: ErrorCategory,
    pub detail: Option<String>,
}

impl ApiError {
    pub fn new(code: i32, category: ErrorCategory) -> Self {
        Self {
            code,
            category,
            detail: None,
        }
    }

    pub fn with_detail(code: i32, category: ErrorCategory, detail: impl Into<String>) -> Self {
        Self {
            code,
            category,
            detail: Some(detail.into()),
        }
    }
}

/// Values used by the small generic backend boundary. Generated SDK methods
/// normally expose concrete Rust types and use this enum only at transport
/// edges or in tests.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Unit,
    Bool(bool),
    I32(i32),
    I64(i64),
    U32(u32),
    U64(u64),
    F32(f32),
    String(String),
    Bytes(Vec<u8>),
    List(Vec<Value>),
    Record(BTreeMap<String, Value>),
    Handle(u64),
}

/// Convert an owned public Rust value to the transport-neutral value used at
/// the backend boundary.  Native and Wasm transports can replace the backend
/// without making module code depend on a pointer-bearing NativeInterface
/// structure.
pub trait IntoValue {
    fn into_value(self) -> Value;
}

/// Convert a backend value back to an owned public Rust value.
pub trait FromValue: Sized {
    fn from_value(value: Value) -> Result<Self, ApiError>;
}

fn value_type_error(expected: &'static str) -> ApiError {
    ApiError::with_detail(100, ErrorCategory::Internal, format!("expected {expected}"))
}

impl IntoValue for Value {
    fn into_value(self) -> Value {
        self
    }
}

impl FromValue for Value {
    fn from_value(value: Value) -> Result<Self, ApiError> {
        Ok(value)
    }
}

impl IntoValue for () {
    fn into_value(self) -> Value {
        Value::Unit
    }
}

impl FromValue for () {
    fn from_value(value: Value) -> Result<Self, ApiError> {
        match value {
            Value::Unit => Ok(()),
            _ => Err(value_type_error("unit")),
        }
    }
}

impl IntoValue for bool {
    fn into_value(self) -> Value {
        Value::Bool(self)
    }
}

impl FromValue for bool {
    fn from_value(value: Value) -> Result<Self, ApiError> {
        match value {
            Value::Bool(value) => Ok(value),
            _ => Err(value_type_error("bool")),
        }
    }
}

macro_rules! impl_integer_values {
    ($type:ty, $variant:ident, $expected:literal) => {
        impl IntoValue for $type {
            fn into_value(self) -> Value {
                Value::$variant(self as _)
            }
        }

        impl FromValue for $type {
            fn from_value(value: Value) -> Result<Self, ApiError> {
                match value {
                    Value::I32(value) => {
                        <$type>::try_from(value).map_err(|_| value_type_error($expected))
                    }
                    Value::I64(value) => {
                        <$type>::try_from(value).map_err(|_| value_type_error($expected))
                    }
                    Value::U32(value) => {
                        <$type>::try_from(value).map_err(|_| value_type_error($expected))
                    }
                    Value::U64(value) => {
                        <$type>::try_from(value).map_err(|_| value_type_error($expected))
                    }
                    _ => Err(value_type_error($expected)),
                }
            }
        }
    };
}

impl_integer_values!(i8, I32, "i8");
impl_integer_values!(i16, I32, "i16");
impl_integer_values!(i32, I32, "i32");
impl_integer_values!(i64, I64, "i64");
impl_integer_values!(u8, U32, "u8");
impl_integer_values!(u16, U32, "u16");
impl_integer_values!(u32, U32, "u32");
impl_integer_values!(u64, U64, "u64");

impl IntoValue for f32 {
    fn into_value(self) -> Value {
        Value::F32(self)
    }
}

impl FromValue for f32 {
    fn from_value(value: Value) -> Result<Self, ApiError> {
        match value {
            Value::F32(value) => Ok(value),
            _ => Err(value_type_error("f32")),
        }
    }
}

impl IntoValue for String {
    fn into_value(self) -> Value {
        Value::String(self)
    }
}

impl IntoValue for &str {
    fn into_value(self) -> Value {
        Value::String(self.to_owned())
    }
}

impl FromValue for String {
    fn from_value(value: Value) -> Result<Self, ApiError> {
        match value {
            Value::String(value) => Ok(value),
            _ => Err(value_type_error("string")),
        }
    }
}

impl<T: IntoValue> IntoValue for Vec<T> {
    fn into_value(self) -> Value {
        Value::List(self.into_iter().map(IntoValue::into_value).collect())
    }
}

impl<T: FromValue> FromValue for Vec<T> {
    fn from_value(value: Value) -> Result<Self, ApiError> {
        let Value::List(values) = value else {
            return Err(value_type_error("list"));
        };
        values.into_iter().map(T::from_value).collect()
    }
}

impl<T: IntoValue> IntoValue for Option<T> {
    fn into_value(self) -> Value {
        self.map(IntoValue::into_value).unwrap_or(Value::Unit)
    }
}

impl<T: FromValue> FromValue for Option<T> {
    fn from_value(value: Value) -> Result<Self, ApiError> {
        match value {
            Value::Unit => Ok(None),
            value => T::from_value(value).map(Some),
        }
    }
}

impl<T: IntoValue, const N: usize> IntoValue for [T; N] {
    fn into_value(self) -> Value {
        Value::List(self.into_iter().map(IntoValue::into_value).collect())
    }
}

impl<T: FromValue, const N: usize> FromValue for [T; N] {
    fn from_value(value: Value) -> Result<Self, ApiError> {
        let values = Vec::<T>::from_value(value)?;
        values
            .try_into()
            .map_err(|_| value_type_error("fixed-size list"))
    }
}

/// A transport implementation for a module instance.
pub trait CalloutBackend {
    fn call(&mut self, function: &str, arguments: &[Value]) -> Result<Value, ApiError>;
}

/// Deterministic per-instance execution accounting.
#[derive(Debug, Clone)]
pub struct ExecutionBudget {
    instruction_fuel: u64,
    host_work: u64,
    host_work_limit: u64,
    result_bytes_limit: usize,
    host_call_depth: u32,
    callback_depth: u32,
}

impl ExecutionBudget {
    pub fn new(instruction_fuel: u64, host_work_limit: u64, result_bytes_limit: usize) -> Self {
        Self {
            instruction_fuel,
            host_work: 0,
            host_work_limit,
            result_bytes_limit,
            host_call_depth: 0,
            callback_depth: 0,
        }
    }

    pub fn charge_guest(&mut self, fuel: u64) -> Result<(), ApiError> {
        if fuel > self.instruction_fuel {
            return Err(ApiError::new(1, ErrorCategory::ResourceLimit));
        }
        self.instruction_fuel -= fuel;
        Ok(())
    }

    pub fn charge_host(&mut self, work: u64) -> Result<(), ApiError> {
        self.host_work = self
            .host_work
            .checked_add(work)
            .ok_or_else(|| ApiError::new(2, ErrorCategory::ResourceLimit))?;
        if self.host_work > self.host_work_limit {
            return Err(ApiError::new(3, ErrorCategory::ResourceLimit));
        }
        Ok(())
    }

    pub fn check_result_size(&self, bytes: usize) -> Result<(), ApiError> {
        if bytes > self.result_bytes_limit {
            Err(ApiError::new(4, ErrorCategory::ResourceLimit))
        } else {
            Ok(())
        }
    }

    pub fn enter_import(&mut self, allow_reentry: bool) -> Result<ImportGuard<'_>, ApiError> {
        if self.host_call_depth != 0 && !allow_reentry {
            return Err(ApiError::new(5, ErrorCategory::ReentryDenied));
        }
        self.host_call_depth = self.host_call_depth.saturating_add(1);
        Ok(ImportGuard { budget: self })
    }

    pub fn enter_callback(&mut self, reentrant: bool) -> Result<CallbackGuard<'_>, ApiError> {
        if self.host_call_depth != 0 && !reentrant {
            return Err(ApiError::new(6, ErrorCategory::ReentryDenied));
        }
        self.callback_depth = self.callback_depth.saturating_add(1);
        Ok(CallbackGuard { budget: self })
    }

    pub fn instruction_fuel(&self) -> u64 {
        self.instruction_fuel
    }

    pub fn host_work(&self) -> u64 {
        self.host_work
    }

    pub fn callback_depth(&self) -> u32 {
        self.callback_depth
    }
}

pub struct ImportGuard<'a> {
    budget: &'a mut ExecutionBudget,
}

impl ImportGuard<'_> {
    pub fn reenter(&mut self, allow_reentry: bool) -> Result<ImportGuard<'_>, ApiError> {
        self.budget.enter_import(allow_reentry)
    }
}

impl Drop for ImportGuard<'_> {
    fn drop(&mut self) {
        self.budget.host_call_depth = self.budget.host_call_depth.saturating_sub(1);
    }
}

pub struct CallbackGuard<'a> {
    budget: &'a mut ExecutionBudget,
}

impl Drop for CallbackGuard<'_> {
    fn drop(&mut self) {
        self.budget.callback_depth = self.budget.callback_depth.saturating_sub(1);
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct InstanceId(pub u64);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ResourceEntry {
    owner: InstanceId,
    family: &'static str,
    generation: u32,
}

/// Instance-owned opaque resource table. Handles include a generation so a
/// dropped handle cannot become valid again when its slot is reused.
#[derive(Debug, Default)]
pub struct ResourceTable {
    entries: Vec<Option<ResourceEntry>>,
    generations: Vec<u32>,
}

impl ResourceTable {
    pub fn insert(&mut self, owner: InstanceId, family: &'static str) -> Result<u64, ApiError> {
        let slot = self
            .entries
            .iter()
            .position(Option::is_none)
            .unwrap_or_else(|| {
                self.entries.push(None);
                self.generations.push(0);
                self.entries.len() - 1
            });
        let generation = self.generations[slot].wrapping_add(1).max(1);
        self.generations[slot] = generation;
        self.entries[slot] = Some(ResourceEntry {
            owner,
            family,
            generation,
        });
        Ok((u64::from(generation) << 32) | slot as u64)
    }

    pub fn validate(
        &self,
        handle: u64,
        owner: InstanceId,
        family: &'static str,
    ) -> Result<(), ApiError> {
        let slot = handle as usize & 0xffff_ffff;
        let generation = (handle >> 32) as u32;
        let Some(Some(entry)) = self.entries.get(slot) else {
            return Err(ApiError::new(7, ErrorCategory::InvalidHandle));
        };
        if entry.owner != owner || entry.family != family || entry.generation != generation {
            return Err(ApiError::new(8, ErrorCategory::InvalidHandle));
        }
        Ok(())
    }

    pub fn drop_handle(
        &mut self,
        handle: u64,
        owner: InstanceId,
        family: &'static str,
    ) -> Result<(), ApiError> {
        self.validate(handle, owner, family)?;
        self.entries[handle as usize & 0xffff_ffff] = None;
        Ok(())
    }

    pub fn len(&self) -> usize {
        self.entries.iter().filter(|entry| entry.is_some()).count()
    }
}

pub type CallbackId = u32;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CallbackPolicy {
    pub reentrant: bool,
}

struct CallbackEntry {
    policy: CallbackPolicy,
    callback: Box<dyn FnMut(&[Value]) -> Result<(), ApiError>>,
}

/// Synchronous callback registry scoped to one module instance.
#[derive(Default)]
pub struct CallbackRegistry {
    next_id: CallbackId,
    callbacks: BTreeMap<CallbackId, CallbackEntry>,
}

impl CallbackRegistry {
    pub fn register<F>(&mut self, policy: CallbackPolicy, callback: F) -> CallbackId
    where
        F: FnMut(&[Value]) -> Result<(), ApiError> + 'static,
    {
        self.next_id = self.next_id.wrapping_add(1).max(1);
        let id = self.next_id;
        self.callbacks.insert(
            id,
            CallbackEntry {
                policy,
                callback: Box::new(callback),
            },
        );
        id
    }

    pub fn invoke(
        &mut self,
        id: CallbackId,
        arguments: &[Value],
        budget: &mut ExecutionBudget,
    ) -> Result<(), ApiError> {
        let entry = self
            .callbacks
            .get_mut(&id)
            .ok_or_else(|| ApiError::new(9, ErrorCategory::InvalidHandle))?;
        let _guard = budget.enter_callback(entry.policy.reentrant)?;
        (entry.callback)(arguments)
    }

    pub fn remove(&mut self, id: CallbackId) -> bool {
        self.callbacks.remove(&id).is_some()
    }

    pub fn clear(&mut self) {
        self.callbacks.clear();
    }

    pub fn len(&self) -> usize {
        self.callbacks.len()
    }
}

/// Common lifecycle state for a module instance.
pub struct InstanceState {
    pub id: InstanceId,
    pub environment: Environment,
    pub budget: ExecutionBudget,
    pub resources: ResourceTable,
    pub callbacks: CallbackRegistry,
    pub fault: Option<ApiError>,
}

impl InstanceState {
    pub fn new(id: InstanceId, environment: Environment, budget: ExecutionBudget) -> Self {
        Self {
            id,
            environment,
            budget,
            resources: ResourceTable::default(),
            callbacks: CallbackRegistry::default(),
            fault: None,
        }
    }

    pub fn fault(&mut self, error: ApiError) {
        self.fault = Some(error);
    }

    pub fn is_faulted(&self) -> bool {
        self.fault.is_some()
    }
}

/// One module's event surface. The engine dispatches events in configured
/// order and never shares this state between synced and unsynced instances.
pub trait ModuleInstance {
    fn state(&mut self) -> &mut InstanceState;
    fn callin(&mut self, name: &str, arguments: &[Value]) -> Result<Value, ApiError>;
}

pub struct Dispatch<'a> {
    modules: Vec<&'a mut dyn ModuleInstance>,
}

impl<'a> Dispatch<'a> {
    pub fn new(modules: Vec<&'a mut dyn ModuleInstance>) -> Self {
        Self { modules }
    }

    pub fn callin(&mut self, name: &str, arguments: &[Value]) -> Vec<Result<Value, ApiError>> {
        self.modules
            .iter_mut()
            .map(|module| module.callin(name, arguments))
            .collect()
    }
}

pub fn environment_set(environment: Environment) -> BTreeSet<Environment> {
    BTreeSet::from([environment])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn synced_and_unsynced_state_are_separate() {
        let mut synced = InstanceState::new(
            InstanceId(1),
            Environment::RulesSynced,
            ExecutionBudget::new(100, 100, 1024),
        );
        let unsynced = InstanceState::new(
            InstanceId(2),
            Environment::RulesUnsynced,
            ExecutionBudget::new(100, 100, 1024),
        );
        let handle = synced.resources.insert(synced.id, "test").unwrap();
        assert!(unsynced
            .resources
            .validate(handle, unsynced.id, "test")
            .is_err());
        assert_eq!(synced.resources.len(), 1);
        assert_eq!(unsynced.resources.len(), 0);
    }

    #[test]
    fn generation_prevents_use_after_drop() {
        let owner = InstanceId(1);
        let mut resources = ResourceTable::default();
        let first = resources.insert(owner, "texture").unwrap();
        resources.drop_handle(first, owner, "texture").unwrap();
        let second = resources.insert(owner, "texture").unwrap();
        assert_ne!(first, second);
        assert!(resources.validate(first, owner, "texture").is_err());
        assert!(resources.validate(second, owner, "texture").is_ok());
    }

    #[test]
    fn import_reentry_is_denied_until_callback_policy_allows_it() {
        let mut budget = ExecutionBudget::new(10, 10, 10);
        {
            let mut guard = budget.enter_import(false).unwrap();
            // A second import would be attempted while the first canonical
            // return/host call is still active; the borrow is scoped exactly
            // like the runtime guard.
            assert!(guard.reenter(false).is_err());
            let nested = guard.reenter(true).unwrap();
            drop(nested);
        }
        assert!(budget.enter_import(false).is_ok());
    }

    #[test]
    fn callbacks_are_scoped_and_budgeted() {
        let mut registry = CallbackRegistry::default();
        let mut budget = ExecutionBudget::new(10, 10, 10);
        let id = registry.register(CallbackPolicy { reentrant: true }, |_| Ok(()));
        registry.invoke(id, &[], &mut budget).unwrap();
        assert_eq!(registry.len(), 1);
        assert_eq!(budget.callback_depth(), 0);
        registry.clear();
        assert_eq!(registry.len(), 0);
    }
}

// The generated façade is part of the public crate so downstream module
// builds do not need to copy transport glue by hand.  The generator/CI job
// refreshes this checked-in artifact from NativeInterface headers.
include!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../rts/wasm/generated/sdk/generated.rs"
));
