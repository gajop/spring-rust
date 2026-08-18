/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include "NativeInterface.h"
#include "WasmInterface/WasmHost.h"

class WasmModule;

// The adapter owns no NativeInterface data. It only copies query inputs into
// native query structs and copies result values into owned WasmValue objects.
// Additional generated adapters can be added behind this same boundary.
class NativeInterfaceWasmAdapter final : public WasmHostAdapter {
public:
	explicit NativeInterfaceWasmAdapter(NativeInterface* nativeInterface)
		: nativeInterface(nativeInterface)
	{
	}

	bool Callout(std::string_view module, std::string_view function,
		const std::vector<WasmValue>& arguments, WasmValue& result,
		std::string& error) override;
	bool Callout(WasmModule& owner, std::string_view module, std::string_view function,
		const std::vector<WasmValue>& arguments, WasmValue& result,
		std::string& error) override;
	const void* ResolveCallout(std::string_view module, std::string_view function) override;
	bool Callout(WasmModule& owner, const void* resolved, std::string_view module,
		std::string_view function, const std::vector<WasmValue>& arguments,
		WasmValue& result, std::string& error) override;
	void* NativeInterfaceHandle() override { return nativeInterface; }

private:
	bool CalloutImpl(WasmModule* owner, const void* resolved, std::string_view module,
		std::string_view function, const std::vector<WasmValue>& arguments,
		WasmValue& result, std::string& error);

	NativeInterface* nativeInterface = nullptr;
};
