Find functions exposed to Lua located in the rts/Lua folder. They are registered via REGISTER_LUA_CFUNC.
After you have found the functions, look at their implementations to extract the C signature. Most of these functions will have various luaL_check* calls, that define the arguments they expect.
Generate C declarations for each registered Lua function.
Start with LuaArchive.cpp