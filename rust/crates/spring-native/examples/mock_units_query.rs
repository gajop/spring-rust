use spring_native::{sys, NativeInterfaceRef, RectangleQueryExt};
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
    let units_query_api = sys::UnitsQueryApi {
        ValidUnitID: Some(mock_valid_unit_id),
        GetAllUnits: None,
        GetTeamUnits: None,
        GetTeamUnitsSorted: None,
        GetTeamUnitsCounts: None,
        GetTeamUnitsByDefs: None,
        GetTeamUnitDefCount: None,
        GetTeamUnitCount: None,
        GetUnitsInRectangle: Some(mock_get_units_in_rectangle),
        GetUnitsInBox: None,
        GetUnitsInPlanes: None,
        GetUnitsInSphere: None,
        GetUnitsInCylinder: None,
        GetUnitArrayCentroid: None,
        GetUnitMapCentroid: None,
        GetUnitNearestAlly: None,
        GetUnitNearestEnemy: None,
        GetUnitSeparation: Some(mock_get_unit_separation),
    };

    let mut units_info_api = sys::UnitsInfoApi::default();
    units_info_api.GetUnitTooltip = Some(mock_get_unit_tooltip);
    units_info_api.GetUnitIsActive = Some(mock_get_unit_is_active);

    let native_interface = sys::NativeInterface {
        memory: ptr::null(),
        game: ptr::null(),
        terrain: ptr::null(),
        teams: ptr::null(),
        unitsQuery: &units_query_api,
        unitsInfo: &units_info_api,
        unitsWeapons: ptr::null(),
        unitsCommands: ptr::null(),
        unitsPieces: ptr::null(),
        features: ptr::null(),
        projectiles: ptr::null(),
        los: ptr::null(),
        unitDefs: ptr::null(),
        featureDefs: ptr::null(),
        weaponDefs: ptr::null(),
        metalMap: ptr::null(),
        pathFinder: ptr::null(),
        rulesParams: ptr::null(),
        mathExtra: ptr::null(),
        moveCtrl: ptr::null(),
        syncedCtrl: ptr::null(),
        cameraApi: ptr::null(),
        input: ptr::null(),
        display: ptr::null(),
        selection: ptr::null(),
        vfs: ptr::null(),
        soundApi: ptr::null(),
        messages: ptr::null(),
        config: ptr::null(),
        tracing: ptr::null(),
        utils: ptr::null(),
        player: ptr::null(),
    };

    let iface = unsafe { NativeInterfaceRef::from_ptr(&native_interface) }.expect("interface ptr");
    let units = iface.units_query().expect("units api available");

    let valid = units.valid_unit_id(5).expect("valid unit call");
    println!("unit 5 valid? {}", valid);

    let rect = (0.0, 0.0, 512.0, 512.0).rect();
    let filter = sys::UnitFilterParams {
        filter: sys::UnitFilter_UNIT_FILTER_ALL,
        teamID: -1,
        allyTeamID: -1,
    };
    let units_in_rect = units
        .get_units_in_rectangle(rect, filter)
        .expect("rectangle query");
    println!("units in rect: {:?}", units_in_rect);

    let separation = units
        .get_unit_separation(10, 20, true, false)
        .expect("separation query");
    println!("unit separation = {}", separation);

    let info = iface.units_info().expect("units info api available");
    let tooltip = info
        .get_unit_tooltip(5)
        .expect("tooltip query")
        .unwrap_or_else(|| "<none>".into());
    println!("tooltip: {}", tooltip);
    let is_active = info.get_unit_is_active(5).expect("active query");
    println!("unit active? {}", is_active);
}
