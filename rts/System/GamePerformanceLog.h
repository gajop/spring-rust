/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#pragma once

#include <array>
#include <chrono>
#include <cstdlib>
#include <cstdint>
#include <fstream>
#include <iomanip>
#include <mutex>
#include <stdexcept>
#include <string_view>
#include <map>
#include <string>
#include <vector>

// Opt-in diagnostic instrumentation, shared by Lua and Wasm. Normal games pay
// one cached flag check and never read a clock or allocate a profiling record.
namespace spring::game_perf {
using Clock = std::chrono::steady_clock;
enum class Kind { Lua, Wasm, Import, Count };
inline bool Enabled() {
	static const bool enabled = [] { const char* p = std::getenv("SPRING_PERF_LOG"); return p != nullptr && *p != '\0'; }();
	return enabled;
}
struct Totals { std::array<double, 3> microseconds{}; std::array<std::uint64_t, 3> calls{}; };
inline thread_local Totals totals;
enum class Event { Created, Destroyed, Fired, Damaged, Commands, Ceg, Drawn };
inline thread_local std::array<std::uint64_t,7> events{};
inline thread_local double damageTotal = 0;
inline void Count(Event event, double damage = 0) {
	if (!Enabled()) return;
	++events[static_cast<unsigned>(event)];
	if (event==Event::Damaged) damageTotal += damage;
}
inline thread_local unsigned depth[3]{};
class Scope {
public:
	explicit Scope(Kind kind): kind(static_cast<unsigned>(kind)), active(Enabled()) {
		if (!active) return;
		++totals.calls[this->kind];
		outer = depth[this->kind]++ == 0;
		if (outer) start = Clock::now();
	}
	~Scope() {
		if (!active) return;
		--depth[kind];
		if (outer) totals.microseconds[kind] += std::chrono::duration<double, std::micro>(Clock::now()-start).count();
	}
private:
	unsigned kind;
	bool active;
	bool outer = false;
	Clock::time_point start;
};
inline std::ofstream& Log() {
	static std::ofstream log = [] {
		std::ofstream stream(std::getenv("SPRING_PERF_LOG"), std::ios::trunc);
		if (!stream) throw std::runtime_error("Cannot open SPRING_PERF_LOG");
		stream << "kind,frame,elapsed_us,lua_us,wasm_us,import_us,lua_calls,wasm_calls,imports,units,projectiles,state_digest,created,destroyed,weapon_fires,damage_events,commands,cegs,drawn,damage_total,features,particles\n";
		return stream;
	}();
	return log;
}
inline void Row(std::string_view kind, int frame, double elapsed, const Totals& delta,
	std::size_t units, std::size_t projectiles, std::uint64_t digest, const std::array<std::uint64_t,7>& eventDelta, double damage, std::size_t features, std::size_t particles)
{
	static std::mutex mutex;
	std::lock_guard lock(mutex);
	auto& log = Log();
	log << kind << ',' << frame << ',' << std::setprecision(12) << elapsed;
	for (const auto value: delta.microseconds) log << ',' << value;
	for (const auto value: delta.calls) log << ',' << value;
	log << ',' << units << ',' << projectiles << ',' << digest;
	for (auto n: eventDelta) log << ',' << n;
	log << ',' << damage << ',' << features << ',' << particles << '\n';
	if (frame % 30 == 0) log.flush();
}
class Frame {
public:
	Frame(std::string_view kind, int frame): kind(kind), frame(frame), active(Enabled()) {
		if (active) { (void)Log(); before = totals; beforeEvents = events; beforeDamage = damageTotal; start = Clock::now(); }
	}
	void Finish(std::size_t units, std::size_t projectiles, std::uint64_t digest = 0, std::size_t features = 0, std::size_t particles = 0) {
		if (!active) return;
		active = false;
		const double elapsed = std::chrono::duration<double, std::micro>(Clock::now()-start).count();
		Totals delta = totals;
		for (unsigned i=0; i<3; ++i) { delta.microseconds[i] -= before.microseconds[i]; delta.calls[i] -= before.calls[i]; }
		auto eventDelta = events;
		for (unsigned i=0;i<7;++i) eventDelta[i]-=beforeEvents[i];
		Row(kind, frame, elapsed, delta, units, projectiles, digest, eventDelta, damageTotal-beforeDamage, features, particles);
	}
	~Frame() { Finish(0, 0); }
private:
	std::string_view kind;
	int frame;
	bool active;
	Clock::time_point start;
	Totals before;
	std::array<std::uint64_t,7> beforeEvents{};
	double beforeDamage = 0;
};
inline const std::vector<std::pair<int,std::string>>& Inputs() {
	static const auto inputs = [] {
		std::vector<std::pair<int,std::string>> result;
		const char* path = std::getenv("SPRING_PERF_INPUT");
		if (path == nullptr || *path == '\0') return result;
		std::ifstream file(path);
		if (!file) throw std::runtime_error("Cannot open SPRING_PERF_INPUT");
		std::string line;
		while (std::getline(file,line)) {
			if (line.empty() || line[0]=='#') continue;
			const auto colon = line.find(':');
			if (colon==std::string::npos) throw std::runtime_error("Expected frame: action in SPRING_PERF_INPUT");
			std::size_t consumed = 0;
			const int frame = std::stoi(line.substr(0,colon),&consumed);
			if (frame<0 || consumed!=colon || (!result.empty() && frame<result.back().first)) throw std::runtime_error("Unordered or invalid input frame");
			result.emplace_back(frame,line.substr(colon+1));
		}
		return result;
	}();
	return inputs;
}
inline int StopFrame() {
	static const int frame = [] {
		const char* value = std::getenv("SPRING_PERF_STOP_FRAME");
		if (value == nullptr) return -1;
		char* end; const long parsed = std::strtol(value, &end, 10);
		if (*end != '\0' || parsed < 0 || parsed > 2147483647) throw std::runtime_error("Invalid SPRING_PERF_STOP_FRAME");
		return static_cast<int>(parsed);
	}();
	return frame;
}
} // namespace spring::game_perf
