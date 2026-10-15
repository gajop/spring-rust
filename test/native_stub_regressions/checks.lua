-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
local done=false
function Update()
	if Spring.GetGameRulesParam('audit_failed')==1 then Spring.SendCommands('quitforce') end
end
function DrawScreen()
	if done or Spring.GetGameRulesParam('audit_synced_ready')~=1 then return end
	done=true
	Spring.InvokeNativeModule('draw')
	Spring.InvokeNativeModule('finish')
	Spring.SendCommands('quitforce')
end
