function gadget:GetInfo()
	local options = Spring.GetModOptions() or {}
	local benchmark = tostring(options.native_api_parity_mode or "") == "benchmark"
	local backend = tostring(options.native_api_parity_benchmark_backend or "")
	local variant = tostring(options.native_api_parity_benchmark_callin_variant or "")
	return {
		name = "Native API variable callin benchmark",
		desc = "Representative string and nested command callins",
		author = "Spring",
		layer = -100,
		enabled = benchmark and backend == "lua" and variant == "variable",
	}
end

local sink = 0

function gadget:AddConsoleLine(message, priority)
	sink = sink + #message + priority
	return false
end

function gadget:CommandNotify(commandID, commandParams, options)
	sink = sink + commandID + #commandParams
	if options ~= nil then
		if options.shift then sink = sink + 1 end
		if options.ctrl then sink = sink + 2 end
		if options.alt then sink = sink + 4 end
	end
	return false
end
