use spring_native::{sys, NativeInterfaceRef};
use std::ptr;

static MOCK_UNITS: [i32; 3] = [101, 202, 303];
static TOOLTIP: &[u8] = b"Mock builder\0";

unsafe extern "C" fn mock_valid_unit_id(
    query: *const sys::ValidUnitIDQuery,
    result: *mut sys::ValidUnitIDResult,
) {
    let query = &*query;
    let result = &mut *result;
    result.error = ptr::null();
    result.valid = query.unitID >= 0;
}

unsafe extern "C" fn mock_get_units_in_rectangle(
    _query: *const sys::GetUnitsInRectangleQuery,
    result: *mut sys::GetUnitsInRectangleResult,
) {
    let result = &mut *result;
    result.error = ptr::null();
    result.units = MOCK_UNITS.as_ptr() as *mut i32;
    result.count = MOCK_UNITS.len() as u32;
}

unsafe extern "C" fn mock_get_unit_separation(
    _query: *const sys::GetUnitSeparationQuery,
    result: *mut sys::GetUnitSeparationResult,
) {
    let result = &mut *result;
    result.error = ptr::null();
    result.separation = 123.0;
}

unsafe extern "C" fn mock_get_unit_tooltip(
    _query: *const sys::GetUnitTooltipQuery,
    result: *mut sys::GetUnitTooltipResult,
) {
    let result = &mut *result;
    result.error = ptr::null();
    result.tooltip = TOOLTIP.as_ptr() as *const i8;
}

unsafe extern "C" fn mock_get_unit_is_active(
    _query: *const sys::GetUnitIsActiveQuery,
    result: *mut sys::GetUnitIsActiveResult,
) {
    let result = &mut *result;
    result.error = ptr::null();
    result.isActive = true;
}

fn main() {
    let mut units_query_api = sys::UnitsQueryApi::default();
    units_query_api.ValidUnitID = Some(mock_valid_unit_id);
    units_query_api.GetUnitsInRectangle = Some(mock_get_units_in_rectangle);
    units_query_api.GetUnitSeparation = Some(mock_get_unit_separation);

    let mut units_info_api = sys::UnitsInfoApi::default();
    units_info_api.GetUnitTooltip = Some(mock_get_unit_tooltip);
    units_info_api.GetUnitIsActive = Some(mock_get_unit_is_active);

    macro_rules! empty_api {
        ($ty:ty) => {
            Box::leak(Box::<$ty>::default()) as *const $ty
        };
    }

    let native_interface = sys::NativeInterface {
        memory: empty_api!(sys::MemoryApi),
        game: empty_api!(sys::GameApi),
        terrain: empty_api!(sys::TerrainApi),
        teams: empty_api!(sys::TeamsApi),
        unitsQuery: &units_query_api,
        unitsInfo: &units_info_api,
        unitsWeapons: empty_api!(sys::UnitsWeaponsApi),
        unitsCommands: empty_api!(sys::UnitsCommandsApi),
        unitsPieces: empty_api!(sys::UnitsPiecesApi),
        features: empty_api!(sys::FeaturesApi),
        projectiles: empty_api!(sys::ProjectilesApi),
        los: empty_api!(sys::LOSApi),
        unitDefs: empty_api!(sys::UnitDefsApi),
        featureDefs: empty_api!(sys::FeatureDefsApi),
        weaponDefs: empty_api!(sys::WeaponDefsApi),
        metalMap: empty_api!(sys::MetalMapApi),
        pathFinder: empty_api!(sys::PathFinderApi),
        platform: empty_api!(sys::PlatformApi),
        rulesParams: empty_api!(sys::RulesParamsApi),
        rmlUi: empty_api!(sys::RmlUiApi),
        mathExtra: empty_api!(sys::MathExtraApi),
        moveCtrl: empty_api!(sys::MoveCtrlApi),
        syncedCtrl: empty_api!(sys::SyncedCtrlApi),
        cameraApi: empty_api!(sys::CameraApi),
        input: empty_api!(sys::InputApi),
        display: empty_api!(sys::DisplayApi),
        selection: empty_api!(sys::SelectionApi),
        vfs: empty_api!(sys::VFSApi),
        soundApi: empty_api!(sys::SoundApi),
        messages: empty_api!(sys::MessagesApi),
        config: empty_api!(sys::ConfigApi),
        tracing: empty_api!(sys::TracingApi),
        utils: empty_api!(sys::UtilsApi),
        player: empty_api!(sys::PlayerApi),
        unsyncedCtrl: empty_api!(sys::UnsyncedCtrlApi),
        unsyncedRead: empty_api!(sys::UnsyncedReadApi),
        lights: empty_api!(sys::LightsApi),
        icons: empty_api!(sys::IconsApi),
        markers: empty_api!(sys::MarkersApi),
        groundDecals: empty_api!(sys::GroundDecalsApi),
        systemControl: empty_api!(sys::SystemControlApi),
        profiling: empty_api!(sys::ProfilingApi),
        gfx: empty_api!(sys::GfxApi),
    };

    let iface = unsafe { NativeInterfaceRef::from_ptr(&native_interface) }.expect("interface ptr");
    let units = iface.units_query();

    let valid = units.valid_unit_id(5).expect("valid unit call");
    println!("unit 5 valid? {}", valid);

    let units_in_rect = units
        .get_units_in_rectangle(0.0, 0.0, 512.0, 512.0, -1)
        .expect("rectangle query");
    println!("units in rect: {:?}", units_in_rect);

    let separation = units
        .get_unit_separation(10, 20, true, false)
        .expect("separation query");
    println!("unit separation = {}", separation);

    let info = iface.units_info();
    let tooltip = info
        .get_unit_tooltip(5)
        .expect("tooltip query")
        .unwrap_or_else(|| "<none>".into());
    println!("tooltip: {}", tooltip);
    let is_active = info.get_unit_is_active(5).expect("active query");
    println!("unit active? {}", is_active);
}
