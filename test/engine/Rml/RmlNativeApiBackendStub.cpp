/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// Native RmlUi data-model tests deliberately link only RmlUi Core, not the
// engine renderer backend. Context ordering is a renderer concern, so provide
// the inert backend edge required to link the complete native API table here.

#include "Rml/Backends/RmlUi_Backend.h"

#include <unordered_map>
#include <unordered_set>

namespace {
	struct ContextOwner {
		void* owner;
		bool menuPhase;
	};
	std::unordered_map<Rml::Context*, ContextOwner> contextOwners;
	bool menuActive = true;
	void* currentOwner = nullptr;
}

bool RmlGui::PullContextToFront(Rml::Context*)
{
	return false;
}

std::vector<std::string> RmlGui::GetDocumentPathRequests(const std::string&)
{
	return {};
}

bool RmlGui::ClearDocumentPathRequests(const std::string&)
{
	return false;
}

bool RmlGui::SetPointerCapture(Rml::Context*, int, int, bool)
{
	return false;
}

bool RmlGui::ProcessPointerCaptureRelease(int, int, int)
{
	return false;
}

bool RmlGui::TakePointerCaptureDelta(Rml::Context*, int&, int&, int&)
{
	return false;
}

void RmlGui::RegisterNativeContext(Rml::Context* context)
{
	contextOwners.insert_or_assign(context, ContextOwner{currentOwner, menuActive});
}

bool RmlGui::IsMenuContext(const Rml::Context* context)
{
	const auto iter = contextOwners.find(const_cast<Rml::Context*>(context));
	return iter != contextOwners.end() && iter->second.menuPhase;
}

void RmlGui::SetMenuActive(bool active)
{
	menuActive = active;
}

void RmlGui::SetCurrentContextOwner(void* owner, bool menuPhase)
{
	currentOwner = owner;
	menuActive = menuPhase;
}

void* RmlGui::GetCurrentContextOwner()
{
	return currentOwner;
}

bool RmlGui::IsCurrentContextMenuPhase()
{
	return menuActive;
}

void* RmlGui::GetContextOwner(const Rml::Context* context)
{
	const auto iter = contextOwners.find(const_cast<Rml::Context*>(context));
	return iter == contextOwners.end() ? nullptr : iter->second.owner;
}

const char* RmlGui::GetAssetVfsModes()
{
	return "";
}

namespace RmlGui {

void ResetTestContextState()
{
	contextOwners.clear();
	menuActive = true;
	currentOwner = nullptr;
}

} // namespace RmlGui
