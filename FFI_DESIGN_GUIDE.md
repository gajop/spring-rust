# Spring Rust FFI API Design Guide

## Core Principles

1. **Global version handshake** - exchange versions once at init, not per-call
2. **Structs by pointer** - all parameters and results passed via pointer
3. **Append-only evolution** - new fields only at end, never reorder/remove
4. **Version checks** - reader checks version before accessing newer fields
5. **Shared scratch buffer** - one thread-local buffer for all dynamic data

## Version Scheme

```c
#define VERSION(major, minor, patch) ((major)*10000 + (minor)*100 + (patch))
```

- **Major**: Breaking changes (field reorder, removal, signature change)
- **Minor**: Additive changes (new fields at end, new functions)
- **Patch**: Bug fixes (no ABI changes)

## Initialization

```c
struct NativeInterface {
    uint32_t hostVersion;  // First field, always present (e.g., 10000 = v1.0.0)

    const TerrainApi* terrain;
    const MetalMapApi* metalMap;
    // ... all API pointers
};

// Client reads host version, decides compatibility
void* InitializeNativeModule(NativeInterface* native) {
    if (MAJOR(native->hostVersion) != CLIENT_MAJOR) {
        return nullptr;  // Incompatible
    }

    if (MINOR(native->hostVersion) < CLIENT_MIN_MINOR) {
        return nullptr;  // Host too old
    }

    g_hostVersion = native->hostVersion;
    return /* client context */;
}
```

## API Function Pattern

```c
// Query struct (input)
struct GetMetalAmountQuery {
    int32_t x;
    int32_t z;
    // v1.2: uint32_t flags;
};

// Result struct (output)
struct GetMetalAmountResult {
    const Error* error;
    float amount;
    // v1.2: float extraction;
};

// API signature (never changes)
struct MetalMapApi {
    void (*GetMetalAmount)(
        const GetMetalAmountQuery* query,
        GetMetalAmountResult* result
    );
};
```

## Error Handling

```c
struct Error {
    int32_t code;
    const char* message;  // Points to static or scratch buffer
};

// Static errors (when no context needed)
static const Error NOT_READY_ERROR = {
    .code = ERROR_NOT_AVAILABLE,
    .message = "Map not ready"
};

// Dynamic errors (with context, use scratch buffer)
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;
static thread_local Error dynamicError;

void NativeGetMetalAmount(const GetMetalAmountQuery* q, GetMetalAmountResult* r) {
    bufferPos = 0;  // Reset scratch buffer

    if (q->x < 0 || q->x >= mapWidth) {
        char* msg = &scratchBuffer[bufferPos];
        bufferPos += snprintf(msg, sizeof(scratchBuffer) - bufferPos,
                             "x coordinate %d out of bounds [0-%d]", q->x, mapWidth-1) + 1;
        dynamicError.code = ERROR_OUT_OF_BOUNDS;
        dynamicError.message = msg;
        r->error = &dynamicError;
        return;
    }

    r->error = nullptr;
    r->amount = metalMap.GetMetalAmount(q->x, q->z);
}
```

## Scratch Buffer for Dynamic Data

**All dynamic data (errors, arrays, strings) uses the shared scratch buffer:**

```cpp
static thread_local char scratchBuffer[1024];
static thread_local size_t bufferPos = 0;

void NativeGetUnitsInArea(const GetUnitsInAreaQuery* q, GetUnitsInAreaResult* r) {
    bufferPos = 0;  // Reset

    // Write array into scratch buffer
    int32_t* unitIds = (int32_t*)&scratchBuffer[bufferPos];
    int count = 0;

    for (auto* unit : units) {
        if (IsInArea(unit, q->x, q->z, q->radius)) {
            unitIds[count++] = unit->id;
            bufferPos += sizeof(int32_t);
        }
    }

    r->error = nullptr;
    r->unitIds = unitIds;
    r->count = count;
}
```

**Lifetime:** Data valid until next call to any API function on same thread.

## Usage Pattern

**Host:**
```cpp
void NativeGetMetalAmount(const GetMetalAmountQuery* q, GetMetalAmountResult* r) {
    bufferPos = 0;
    r->error = nullptr;
    r->amount = metalMap.GetMetalAmount(q->x, q->z);

    if (g_clientVersion >= VERSION(1, 2, 0)) {
        r->extraction = metalMap.GetMetalExtraction(q->x, q->z);
    }
}
```

**Client:**
```rust
let query = GetMetalAmountQuery { x: 100, z: 200 };
let mut result = GetMetalAmountResult::default();

get_metal_amount(&query, &mut result);

if !result.error.is_null() {
    bail!("Error: {}", CStr::from_ptr((*result.error).message));
}

let amount = result.amount;

if g_host_version >= VERSION(1, 2, 0) {
    let extraction = result.extraction;
}
```

## Callback Pattern (Callins)

```c
// Event struct (append-only)
struct UnitCreatedEvent {
    int32_t unitId;
    Float3 position;
    // v1.2: Float3 velocity;
};

// Callback signature (never changes)
typedef void (*UnitCreatedCallback)(const UnitCreatedEvent* event);

// Host implementation
void FireUnitCreated(int32_t unitId, Float3 pos) {
    UnitCreatedEvent event = {};
    event.unitId = unitId;
    event.position = pos;
    // velocity stays uninitialized - client checks version before accessing

    if (g_clientCallback) {
        g_clientCallback(&event);
    }
}
```

Client checks version before accessing newer fields.

## Rules

✅ **Do:**
- Pass structs by pointer
- Append fields at end only
- Check version before accessing newer fields
- Use scratch buffer for dynamic data
- Reset bufferPos at start of each function

❌ **Don't:**
- Pass structs by value
- Reorder/remove fields (major version bump)
- Access fields without version check
- Allocate memory (malloc/new/std::vector)
- Echo input in output

## Trade-offs

### Benefits
- Never break existing clients - old mods keep working
- Extend APIs without coordination - add fields independently
- Simple version model - one integer comparison
- Plugin ecosystem stability - critical for mods
- Zero allocation overhead - scratch buffer reused

### Costs
- More verbose - struct construction overhead
- Version discipline required - must check before accessing fields
- Client adaptation complexity - supporting multiple host versions is work
- Single scratch buffer per thread - must reset per call
- No automatic validation - version checks are manual

### When NOT to use this pattern
- Internal-only APIs - if no external plugins, simpler patterns work
- Stable APIs that never change - versioning overhead unnecessary

### Why it's worth it here
- Plugin ecosystem - Rust mods are external, need stability
- Long-lived codebase - Spring engine evolves over years
- Multiple mod versions - users don't update everything at once
- Industry standard - Vulkan, Win32, DirectX all use this pattern
