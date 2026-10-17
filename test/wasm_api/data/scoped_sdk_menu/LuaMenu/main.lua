-- This file is part of the Spring engine (GPL v2 or later), see LICENSE.html
local previous = ""
function Update()
    local current = Spring.GetConfigString("ScopedSdkProof", "")
    if current == "init-failure-cleaned" then
        Spring.Echo("[ScopedSdkProof] init-failure-cleaned")
        Spring.Quit()
        return
    end
    if current ~= previous then
        Spring.Echo("[ScopedSdkProof] " .. current)
        previous = current
    end
end

function AllowDraw() return true end
