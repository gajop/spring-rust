/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

// Native RmlUi data-model tests deliberately link only RmlUi Core, not the
// engine renderer backend. Context ordering is a renderer concern, so provide
// the inert backend edge required to link the complete native API table here.

#include "Rml/Backends/RmlUi_Backend.h"

bool RmlGui::PullContextToFront(Rml::Context*)
{
	return false;
}

bool RmlGui::SetPointerCapture(Rml::Context*, int, int, bool)
{
	return false;
}
