/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

// Every texture, renderbuffer and buffer the GL context holds, found by name and
// sized from the driver's own level queries (GL 4.5 direct-state access), logged
// as "[gpu-memory]" lines: totals by kind, then the largest groups of objects
// that share a target, format and size, with their GL labels. `minKiB` sets the
// smallest group listed. A diagnostic: it scans every name and costs milliseconds.
void LogGpuMemoryCensus(int minKiB = 256);
