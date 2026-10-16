-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
local done=false
function Update()
	if Spring.GetGameRulesParam('audit_failed')==1 then Spring.SendCommands('quitforce') end
end
function DrawScreen()
	if done or Spring.GetGameRulesParam('audit_synced_ready')~=1 then return end
	done=true
	for _,index in ipairs({-1,0}) do
		for _,usable in ipairs({false,true}) do
			local values={Spring.GetScreenGeometry(index,usable)}
			Spring.InvokeNativeModule('display '..index..' '..(usable and 1 or 0)..' '..table.concat(values,' '))
		end
	end
	local available,ground=Spring.HaveAdvShading()
	Spring.InvokeNativeModule('shading '..(available and 1 or 0)..' '..(ground and 1 or 0))
	Spring.InvokeNativeModule('files')
	Spring.InvokeNativeModule('commands')
	Spring.InvokeNativeModule('draw')
	Spring.InvokeNativeModule('finish')
	Spring.SendCommands('quitforce')
end
