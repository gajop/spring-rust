#pragma once

#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// ============================================================================
// Constants API
// @see rts/Lua/LuaConst*.cpp
//
// Game, engine, and command constants
// ============================================================================

#if !defined(__cplusplus) || defined(SPRING_NATIVE_BINDGEN)
enum CommandID {
	CMD_STOP = 0,
	CMD_INSERT = 1,
	CMD_REMOVE = 2,
	CMD_WAIT = 5,
	CMD_TIMEWAIT = 6,
	CMD_DEATHWAIT = 7,
	CMD_SQUADWAIT = 8,
	CMD_GATHERWAIT = 9,
	CMD_MOVE = 10,
	CMD_PATROL = 15,
	CMD_FIGHT = 16,
	CMD_ATTACK = 20,
	CMD_AREA_ATTACK = 21,
	CMD_GUARD = 25,
	CMD_AISELECT = 30,
	CMD_GROUPSELECT = 35,
	CMD_GROUPADD = 36,
	CMD_GROUPCLEAR = 37,
	CMD_REPAIR = 40,
	CMD_FIRE_STATE = 45,
	CMD_MOVE_STATE = 50,
	CMD_SETBASE = 55,
	CMD_INTERNAL = 60,
	CMD_SELFD = 65,
	CMD_SET_WANTED_MAX_SPEED = 70,
	CMD_LOAD_UNITS = 75,
	CMD_LOAD_ONTO = 76,
	CMD_UNLOAD_UNITS = 80,
	CMD_UNLOAD_UNIT = 81,
	CMD_ONOFF = 85,
	CMD_RECLAIM = 90,
	CMD_CLOAK = 95,
	CMD_STOCKPILE = 100,
	CMD_MANUALFIRE = 105,
	CMD_RESTORE = 110,
	CMD_REPEAT = 115,
	CMD_TRAJECTORY = 120,
	CMD_RESURRECT = 125,
	CMD_CAPTURE = 130,
	CMD_AUTOREPAIRLEVEL = 135,
	CMD_LOOPBACKATTACK = 140,
	CMD_IDLEMODE = 145,
	CMD_FAILED = 150,
};

// Command options (bitfield)
enum CommandOption {
	CMD_OPT_INTERNAL = (1 << 0),
	CMD_OPT_RIGHT = (1 << 1),
	CMD_OPT_SHIFT = (1 << 2),
	CMD_OPT_CTRL = (1 << 3),
	CMD_OPT_ALT = (1 << 4),
	CMD_OPT_META = (1 << 5),
};

// Fire state
enum FireState {
	FIRESTATE_HOLDFIRE = 0,
	FIRESTATE_RETURNFIRE = 1,
	FIRESTATE_FIREATWILL = 2,
	FIRESTATE_FIREATNEUTRAL = 3,
};

// Move state
enum MoveState {
	MOVESTATE_HOLDPOS = 0,
	MOVESTATE_MANEUVER = 1,
	MOVESTATE_ROAM = 2,
};
#endif

// Unit categories (bitfield)
enum UnitCategory {
	CAT_LAND = (1 << 0),
	CAT_SHIP = (1 << 1),
	CAT_HOVER = (1 << 2),
	CAT_AMPHIB = (1 << 3),
	CAT_AIR = (1 << 4),
	CAT_BUILDER = (1 << 5),
	CAT_FACTORY = (1 << 6),
	CAT_WEAPON = (1 << 7),
	CAT_MOBILEUNIT = (1 << 8),
	CAT_NOTAIR = (1 << 9),
};

// Damage types
enum DamageType {
	DAMAGE_EXPLOSION_GENERATOR = -1,
	DAMAGE_COLLISION_OBJECT = -2,
	DAMAGE_COLLISION_GROUND = -3,
	DAMAGE_KILLED = -4,
	DAMAGE_RECLAIMED = -5,
	DAMAGE_SELFD = -6,
	DAMAGE_CRUSHED = -7,
	DAMAGE_FIRE = -8,
};

// COB constants
enum COBConstant {
	COB_ACTIVATION = 1,
	COB_STANDINGMOVEORDERS = 2,
	COB_STANDINGFIREORDERS = 3,
	COB_HEALTH = 4,
	COB_INBUILDSTANCE = 5,
	COB_BUSY = 6,
	COB_PIECE_XZ = 7,
	COB_PIECE_Y = 8,
	COB_UNIT_XZ = 9,
	COB_UNIT_Y = 10,
	COB_UNIT_HEIGHT = 11,
	COB_XZ_ATAN = 12,
	COB_XZ_HYPOT = 13,
	COB_GROUND_HEIGHT = 14,
	COB_BUILD_PERCENT_LEFT = 15,
};

// Engine/game constants mirrored from Lua's Game.* constant entries
// (rts/Lua/LuaConstGame.cpp). Values that depend on a loaded map, game, or
// runtime state are exposed through GameApi getters instead.
enum GameConstant {
	GAME_MAX_TEAMS = 255,
	GAME_MAX_PLAYERS = 251,
	GAME_MAX_AIS = 255,
	GAME_MAX_UNITS = 32000,
	GAME_MAX_FEATURES = 32000,
	GAME_MAX_PROJECTILES = 128000,
	GAME_MAX_WEAPONS_PER_UNIT = 32,
	GAME_SQUARE_SIZE = 8,
	GAME_METAL_MAP_SQUARE_SIZE = 16,
	GAME_BUILD_SQUARE_SIZE = 16,
	GAME_BUILD_GRID_RESOLUTION = 2,
	GAME_FOOTPRINT_SCALE = 2,
	GAME_GAME_SPEED = 30,
	GAME_UNIT_SLOWUPDATE_RATE = 15,
	GAME_TEAM_SLOWUPDATE_RATE = 30,
};

// ============================================================================
// OpenGL constants (mirror of rts/Lua/LuaConstGL.cpp GL.* table)
//
// Exposed so native callers get the same values Lua scripts get from GL.*,
// instead of hardcoding hex literals. Enumerators use a GLC_ prefix to avoid
// colliding with the GL_* macros from the GL loader when this header is included
// next to it. Values are the OpenGL spec values; api/Gfx.cpp static_asserts each
// GLC_* against the real GL_* so a wrong literal fails the engine build.
//
// The tail entries (RGBA16F_ARB .. FRAMEBUFFER_UNSUPPORTED_EXT) are the formats /
// FBO targets+status that LuaConstGL.cpp documents as "useful" but does NOT
// register in GL.*; native ReadPixels / CreateFBO / IsValidFBO callers need them.
// (No comments may appear between the X(...) lines below: they are one
// backslash-continued macro and a comment would terminate it early.)
// ============================================================================
#define SPRING_GL_CONSTANTS(X) \
	X(POINTS                         , 0x00000000u) \
	X(LINES                          , 0x00000001u) \
	X(LINE_LOOP                      , 0x00000002u) \
	X(LINE_STRIP                     , 0x00000003u) \
	X(TRIANGLES                      , 0x00000004u) \
	X(TRIANGLE_STRIP                 , 0x00000005u) \
	X(TRIANGLE_FAN                   , 0x00000006u) \
	X(QUADS                          , 0x00000007u) \
	X(QUAD_STRIP                     , 0x00000008u) \
	X(POLYGON                        , 0x00000009u) \
	X(LINE_STRIP_ADJACENCY           , 0x0000000Bu) \
	X(LINES_ADJACENCY                , 0x0000000Au) \
	X(TRIANGLE_STRIP_ADJACENCY       , 0x0000000Du) \
	X(TRIANGLES_ADJACENCY            , 0x0000000Cu) \
	X(PATCHES                        , 0x0000000Eu) \
	X(ZERO                           , 0x00000000u) \
	X(ONE                            , 0x00000001u) \
	X(SRC_COLOR                      , 0x00000300u) \
	X(ONE_MINUS_SRC_COLOR            , 0x00000301u) \
	X(SRC_ALPHA                      , 0x00000302u) \
	X(ONE_MINUS_SRC_ALPHA            , 0x00000303u) \
	X(DST_ALPHA                      , 0x00000304u) \
	X(ONE_MINUS_DST_ALPHA            , 0x00000305u) \
	X(DST_COLOR                      , 0x00000306u) \
	X(ONE_MINUS_DST_COLOR            , 0x00000307u) \
	X(SRC_ALPHA_SATURATE             , 0x00000308u) \
	X(FUNC_ADD                       , 0x00008006u) \
	X(FUNC_SUBTRACT                  , 0x0000800Au) \
	X(FUNC_REVERSE_SUBTRACT          , 0x0000800Bu) \
	X(MIN                            , 0x00008007u) \
	X(MAX                            , 0x00008008u) \
	X(NEVER                          , 0x00000200u) \
	X(LESS                           , 0x00000201u) \
	X(EQUAL                          , 0x00000202u) \
	X(LEQUAL                         , 0x00000203u) \
	X(GREATER                        , 0x00000204u) \
	X(NOTEQUAL                       , 0x00000205u) \
	X(GEQUAL                         , 0x00000206u) \
	X(ALWAYS                         , 0x00000207u) \
	X(KEEP                           , 0x00001E00u) \
	X(INCR                           , 0x00001E02u) \
	X(DECR                           , 0x00001E03u) \
	X(INCR_WRAP                      , 0x00008507u) \
	X(DECR_WRAP                      , 0x00008508u) \
	X(CLEAR                          , 0x00001500u) \
	X(AND                            , 0x00001501u) \
	X(AND_REVERSE                    , 0x00001502u) \
	X(COPY                           , 0x00001503u) \
	X(AND_INVERTED                   , 0x00001504u) \
	X(NOOP                           , 0x00001505u) \
	X(XOR                            , 0x00001506u) \
	X(OR                             , 0x00001507u) \
	X(NOR                            , 0x00001508u) \
	X(EQUIV                          , 0x00001509u) \
	X(INVERT                         , 0x0000150Au) \
	X(OR_REVERSE                     , 0x0000150Bu) \
	X(COPY_INVERTED                  , 0x0000150Cu) \
	X(OR_INVERTED                    , 0x0000150Du) \
	X(NAND                           , 0x0000150Eu) \
	X(SET                            , 0x0000150Fu) \
	X(BACK                           , 0x00000405u) \
	X(FRONT                          , 0x00000404u) \
	X(FRONT_AND_BACK                 , 0x00000408u) \
	X(POINT                          , 0x00001B00u) \
	X(LINE                           , 0x00001B01u) \
	X(FILL                           , 0x00001B02u) \
	X(FLAT                           , 0x00001D00u) \
	X(SMOOTH                         , 0x00001D01u) \
	X(MODELVIEW                      , 0x00001700u) \
	X(PROJECTION                     , 0x00001701u) \
	X(TEXTURE                        , 0x00001702u) \
	X(NEAREST                        , 0x00002600u) \
	X(LINEAR                         , 0x00002601u) \
	X(NEAREST_MIPMAP_NEAREST         , 0x00002700u) \
	X(LINEAR_MIPMAP_NEAREST          , 0x00002701u) \
	X(NEAREST_MIPMAP_LINEAR          , 0x00002702u) \
	X(LINEAR_MIPMAP_LINEAR           , 0x00002703u) \
	X(REPEAT                         , 0x00002901u) \
	X(MIRRORED_REPEAT                , 0x00008370u) \
	X(CLAMP                          , 0x00002900u) \
	X(CLAMP_TO_EDGE                  , 0x0000812Fu) \
	X(CLAMP_TO_BORDER                , 0x0000812Du) \
	X(TEXTURE_ENV                    , 0x00002300u) \
	X(TEXTURE_ENV_MODE               , 0x00002200u) \
	X(TEXTURE_ENV_COLOR              , 0x00002201u) \
	X(MODULATE                       , 0x00002100u) \
	X(DECAL                          , 0x00002101u) \
	X(BLEND                          , 0x00000BE2u) \
	X(REPLACE                        , 0x00001E01u) \
	X(TEXTURE_FILTER_CONTROL         , 0x00008500u) \
	X(TEXTURE_LOD_BIAS               , 0x00008501u) \
	X(TEXTURE_GEN_MODE               , 0x00002500u) \
	X(EYE_PLANE                      , 0x00002502u) \
	X(OBJECT_PLANE                   , 0x00002501u) \
	X(EYE_LINEAR                     , 0x00002400u) \
	X(OBJECT_LINEAR                  , 0x00002401u) \
	X(SPHERE_MAP                     , 0x00002402u) \
	X(NORMAL_MAP                     , 0x00008511u) \
	X(REFLECTION_MAP                 , 0x00008512u) \
	X(S                              , 0x00002000u) \
	X(T                              , 0x00002001u) \
	X(R                              , 0x00002002u) \
	X(Q                              , 0x00002003u) \
	X(CURRENT_BIT                    , 0x00000001u) \
	X(POINT_BIT                      , 0x00000002u) \
	X(LINE_BIT                       , 0x00000004u) \
	X(POLYGON_BIT                    , 0x00000008u) \
	X(POLYGON_STIPPLE_BIT            , 0x00000010u) \
	X(PIXEL_MODE_BIT                 , 0x00000020u) \
	X(LIGHTING_BIT                   , 0x00000040u) \
	X(FOG_BIT                        , 0x00000080u) \
	X(DEPTH_BUFFER_BIT               , 0x00000100u) \
	X(ACCUM_BUFFER_BIT               , 0x00000200u) \
	X(STENCIL_BUFFER_BIT             , 0x00000400u) \
	X(VIEWPORT_BIT                   , 0x00000800u) \
	X(TRANSFORM_BIT                  , 0x00001000u) \
	X(ENABLE_BIT                     , 0x00002000u) \
	X(COLOR_BUFFER_BIT               , 0x00004000u) \
	X(HINT_BIT                       , 0x00008000u) \
	X(EVAL_BIT                       , 0x00010000u) \
	X(LIST_BIT                       , 0x00020000u) \
	X(TEXTURE_BIT                    , 0x00040000u) \
	X(SCISSOR_BIT                    , 0x00080000u) \
	X(FOG_HINT                       , 0x00000C54u) \
	X(PERSPECTIVE_CORRECTION_HINT    , 0x00000C50u) \
	X(DONT_CARE                      , 0x00001100u) \
	X(FASTEST                        , 0x00001101u) \
	X(NICEST                         , 0x00001102u) \
	X(AMBIENT                        , 0x00001200u) \
	X(DIFFUSE                        , 0x00001201u) \
	X(SPECULAR                       , 0x00001202u) \
	X(POSITION                       , 0x00001203u) \
	X(SPOT_DIRECTION                 , 0x00001204u) \
	X(SPOT_EXPONENT                  , 0x00001205u) \
	X(SPOT_CUTOFF                    , 0x00001206u) \
	X(CONSTANT_ATTENUATION           , 0x00001207u) \
	X(LINEAR_ATTENUATION             , 0x00001208u) \
	X(QUADRATIC_ATTENUATION          , 0x00001209u) \
	X(VERTEX_SHADER                  , 0x00008B31u) \
	X(TESS_CONTROL_SHADER            , 0x00008E88u) \
	X(TESS_EVALUATION_SHADER         , 0x00008E87u) \
	X(GEOMETRY_SHADER_EXT            , 0x00008DD9u) \
	X(FRAGMENT_SHADER                , 0x00008B30u) \
	X(GEOMETRY_INPUT_TYPE_EXT        , 0x00008DDBu) \
	X(GEOMETRY_OUTPUT_TYPE_EXT       , 0x00008DDCu) \
	X(GEOMETRY_VERTICES_OUT_EXT      , 0x00008DDAu) \
	X(PATCH_VERTICES                 , 0x00008E72u) \
	X(PATCH_DEFAULT_OUTER_LEVEL      , 0x00008E74u) \
	X(PATCH_DEFAULT_INNER_LEVEL      , 0x00008E73u) \
	X(BYTE                           , 0x00001400u) \
	X(UNSIGNED_BYTE                  , 0x00001401u) \
	X(SHORT                          , 0x00001402u) \
	X(UNSIGNED_SHORT                 , 0x00001403u) \
	X(INT                            , 0x00001404u) \
	X(UNSIGNED_INT                   , 0x00001405u) \
	X(FLOAT                          , 0x00001406u) \
	X(HALF_FLOAT                     , 0x0000140Bu) \
	X(FLOAT_VEC4                     , 0x00008B52u) \
	X(INT_VEC4                       , 0x00008B55u) \
	X(UNSIGNED_INT_VEC4              , 0x00008DC8u) \
	X(FLOAT_MAT4                     , 0x00008B5Cu) \
	X(ELEMENT_ARRAY_BUFFER           , 0x00008893u) \
	X(ARRAY_BUFFER                   , 0x00008892u) \
	X(UNIFORM_BUFFER                 , 0x00008A11u) \
	X(SHADER_STORAGE_BUFFER          , 0x000090D2u) \
	X(TEXTURE_1D                     , 0x00000DE0u) \
	X(TEXTURE_2D                     , 0x00000DE1u) \
	X(TEXTURE_2D_ARRAY               , 0x00008C1Au) \
	X(TEXTURE_3D                     , 0x0000806Fu) \
	X(TEXTURE_CUBE_MAP               , 0x00008513u) \
	X(TEXTURE_CUBE_MAP_POSITIVE_X    , 0x00008515u) \
	X(TEXTURE_CUBE_MAP_NEGATIVE_X    , 0x00008516u) \
	X(TEXTURE_CUBE_MAP_POSITIVE_Y    , 0x00008517u) \
	X(TEXTURE_CUBE_MAP_NEGATIVE_Y    , 0x00008518u) \
	X(TEXTURE_CUBE_MAP_POSITIVE_Z    , 0x00008519u) \
	X(TEXTURE_CUBE_MAP_NEGATIVE_Z    , 0x0000851Au) \
	X(TEXTURE_2D_MULTISAMPLE         , 0x00009100u) \
	X(MAX_IMAGE_UNITS                , 0x00008F38u) \
	X(RGBA32F                        , 0x00008814u) \
	X(RGBA16F                        , 0x0000881Au) \
	X(RG32F                          , 0x00008230u) \
	X(RG16F                          , 0x0000822Fu) \
	X(R11F_G11F_B10F                 , 0x00008C3Au) \
	X(R32F                           , 0x0000822Eu) \
	X(R16F                           , 0x0000822Du) \
	X(RGBA32UI                       , 0x00008D70u) \
	X(RGBA16UI                       , 0x00008D76u) \
	X(RGB10_A2UI                     , 0x0000906Fu) \
	X(RGBA8UI                        , 0x00008D7Cu) \
	X(RG32UI                         , 0x0000823Cu) \
	X(RG16UI                         , 0x0000823Au) \
	X(RG8UI                          , 0x00008238u) \
	X(R32UI                          , 0x00008236u) \
	X(R16UI                          , 0x00008234u) \
	X(R8UI                           , 0x00008232u) \
	X(RGBA32I                        , 0x00008D82u) \
	X(RGBA16I                        , 0x00008D88u) \
	X(RGBA8I                         , 0x00008D8Eu) \
	X(RG32I                          , 0x0000823Bu) \
	X(RG16I                          , 0x00008239u) \
	X(RG8I                           , 0x00008237u) \
	X(R32I                           , 0x00008235u) \
	X(R16I                           , 0x00008233u) \
	X(R8I                            , 0x00008231u) \
	X(RGBA16                         , 0x0000805Bu) \
	X(RGB10_A2                       , 0x00008059u) \
	X(RGBA8                          , 0x00008058u) \
	X(RG16                           , 0x0000822Cu) \
	X(RG8                            , 0x0000822Bu) \
	X(R16                            , 0x0000822Au) \
	X(R8                             , 0x00008229u) \
	X(RGBA16_SNORM                   , 0x00008F9Bu) \
	X(RGBA8_SNORM                    , 0x00008F97u) \
	X(RG16_SNORM                     , 0x00008F99u) \
	X(RG8_SNORM                      , 0x00008F95u) \
	X(R16_SNORM                      , 0x00008F98u) \
	X(R8_SNORM                       , 0x00008F94u) \
	X(DEPTH_COMPONENT16              , 0x000081A5u) \
	X(DEPTH_COMPONENT24              , 0x000081A6u) \
	X(DEPTH_COMPONENT32              , 0x000081A7u) \
	X(DEPTH_COMPONENT32F             , 0x00008CACu) \
	X(READ_ONLY                      , 0x000088B8u) \
	X(WRITE_ONLY                     , 0x000088B9u) \
	X(READ_WRITE                     , 0x000088BAu) \
	X(VERTEX_ATTRIB_ARRAY_BARRIER_BIT, 0x00000001u) \
	X(ELEMENT_ARRAY_BARRIER_BIT      , 0x00000002u) \
	X(UNIFORM_BARRIER_BIT            , 0x00000004u) \
	X(TEXTURE_FETCH_BARRIER_BIT      , 0x00000008u) \
	X(SHADER_IMAGE_ACCESS_BARRIER_BIT, 0x00000020u) \
	X(COMMAND_BARRIER_BIT            , 0x00000040u) \
	X(PIXEL_BUFFER_BARRIER_BIT       , 0x00000080u) \
	X(TEXTURE_UPDATE_BARRIER_BIT     , 0x00000100u) \
	X(BUFFER_UPDATE_BARRIER_BIT      , 0x00000200u) \
	X(FRAMEBUFFER_BARRIER_BIT        , 0x00000400u) \
	X(TRANSFORM_FEEDBACK_BARRIER_BIT , 0x00000800u) \
	X(ATOMIC_COUNTER_BARRIER_BIT     , 0x00001000u) \
	X(SHADER_STORAGE_BARRIER_BIT     , 0x00002000u) \
	X(ALL_BARRIER_BITS               , 0xFFFFFFFFu) \
	X(COLOR_ATTACHMENT0              , 0x00008CE0u) \
	X(COLOR_ATTACHMENT1              , 0x00008CE1u) \
	X(COLOR_ATTACHMENT2              , 0x00008CE2u) \
	X(COLOR_ATTACHMENT3              , 0x00008CE3u) \
	X(COLOR_ATTACHMENT4              , 0x00008CE4u) \
	X(COLOR_ATTACHMENT5              , 0x00008CE5u) \
	X(COLOR_ATTACHMENT6              , 0x00008CE6u) \
	X(COLOR_ATTACHMENT7              , 0x00008CE7u) \
	X(COLOR_ATTACHMENT8              , 0x00008CE8u) \
	X(COLOR_ATTACHMENT9              , 0x00008CE9u) \
	X(COLOR_ATTACHMENT10             , 0x00008CEAu) \
	X(COLOR_ATTACHMENT11             , 0x00008CEBu) \
	X(COLOR_ATTACHMENT12             , 0x00008CECu) \
	X(COLOR_ATTACHMENT13             , 0x00008CEDu) \
	X(COLOR_ATTACHMENT14             , 0x00008CEEu) \
	X(COLOR_ATTACHMENT15             , 0x00008CEFu) \
	X(DEPTH_ATTACHMENT               , 0x00008D00u) \
	X(STENCIL_ATTACHMENT             , 0x00008D20u) \
	X(COLOR_ATTACHMENT0_EXT          , 0x00008CE0u) \
	X(COLOR_ATTACHMENT1_EXT          , 0x00008CE1u) \
	X(COLOR_ATTACHMENT2_EXT          , 0x00008CE2u) \
	X(COLOR_ATTACHMENT3_EXT          , 0x00008CE3u) \
	X(COLOR_ATTACHMENT4_EXT          , 0x00008CE4u) \
	X(COLOR_ATTACHMENT5_EXT          , 0x00008CE5u) \
	X(COLOR_ATTACHMENT6_EXT          , 0x00008CE6u) \
	X(COLOR_ATTACHMENT7_EXT          , 0x00008CE7u) \
	X(COLOR_ATTACHMENT8_EXT          , 0x00008CE8u) \
	X(COLOR_ATTACHMENT9_EXT          , 0x00008CE9u) \
	X(COLOR_ATTACHMENT10_EXT         , 0x00008CEAu) \
	X(COLOR_ATTACHMENT11_EXT         , 0x00008CEBu) \
	X(COLOR_ATTACHMENT12_EXT         , 0x00008CECu) \
	X(COLOR_ATTACHMENT13_EXT         , 0x00008CEDu) \
	X(COLOR_ATTACHMENT14_EXT         , 0x00008CEEu) \
	X(COLOR_ATTACHMENT15_EXT         , 0x00008CEFu) \
	X(DEPTH_ATTACHMENT_EXT           , 0x00008D00u) \
	X(STENCIL_ATTACHMENT_EXT         , 0x00008D20u) \
	X(BUFFER                         , 0x000082E0u) \
	X(SHADER                         , 0x000082E1u) \
	X(PROGRAM                        , 0x000082E2u) \
	X(VERTEX_ARRAY                   , 0x00008074u) \
	X(QUERY                          , 0x000082E3u) \
	X(PROGRAM_PIPELINE               , 0x000082E4u) \
	X(TRANSFORM_FEEDBACK             , 0x00008E22u) \
	X(RENDERBUFFER                   , 0x00008D41u) \
	X(FRAMEBUFFER                    , 0x00008D40u) \
	X(RGBA16F_ARB                                  , 0x0000881Au) \
	X(RGBA32F_ARB                                  , 0x00008814u) \
	X(DEPTH_COMPONENT                              , 0x00001902u) \
	X(RED                                          , 0x00001903u) \
	X(RG                                           , 0x00008227u) \
	X(RGB                                          , 0x00001907u) \
	X(RGBA                                         , 0x00001908u) \
	X(STENCIL_INDEX                                , 0x00001901u) \
	X(FRAMEBUFFER_EXT                              , 0x00008D40u) \
	X(READ_FRAMEBUFFER_EXT                         , 0x00008CA8u) \
	X(DRAW_FRAMEBUFFER_EXT                         , 0x00008CA9u) \
	X(FRAMEBUFFER_COMPLETE_EXT                     , 0x00008CD5u) \
	X(FRAMEBUFFER_INCOMPLETE_ATTACHMENT_EXT        , 0x00008CD6u) \
	X(FRAMEBUFFER_INCOMPLETE_MISSING_ATTACHMENT_EXT, 0x00008CD7u) \
	X(FRAMEBUFFER_INCOMPLETE_DIMENSIONS_EXT        , 0x00008CD9u) \
	X(FRAMEBUFFER_INCOMPLETE_FORMATS_EXT           , 0x00008CDAu) \
	X(FRAMEBUFFER_INCOMPLETE_DRAW_BUFFER_EXT       , 0x00008CDBu) \
	X(FRAMEBUFFER_INCOMPLETE_READ_BUFFER_EXT       , 0x00008CDCu) \
	X(FRAMEBUFFER_UNSUPPORTED_EXT                  , 0x00008CDDu) \
	X(ALL_ATTRIB_BITS                , 0xFFFFFFFFu) \

enum GLConstant {
#define SPRING_GL_ENUM_ENTRY(name, value) GLC_##name = (value),
	SPRING_GL_CONSTANTS(SPRING_GL_ENUM_ENTRY)
#undef SPRING_GL_ENUM_ENTRY
};

#ifdef __cplusplus
}
#endif
