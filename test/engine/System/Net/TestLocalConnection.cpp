/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "System/Net/LocalConnection.h"
#include "Net/Protocol/BaseNetProtocol.h"

#include <catch_amalgamated.hpp>
#include <algorithm>
#include <atomic>
#include <thread>

TEST_CASE("Local connection reports queued packets without consuming them")
{
	netcode::CLocalConnection server;
	netcode::CLocalConnection client;
	auto& protocol = CBaseNetProtocol::Get();
	const auto first = protocol.SendNewFrame();
	const auto second = protocol.SendKeyFrame(30);

	CHECK(client.GetPacketQueueSize() == 0);
	CHECK_FALSE(client.HasIncomingData());
	server.SendData(first);
	server.SendData(second);
	CHECK(server.GetPacketQueueSize() == 0);
	CHECK(client.GetPacketQueueSize() == 2);
	CHECK(client.HasIncomingData());
	CHECK(client.Peek(0) == first);
	CHECK(client.GetPacketQueueSize() == 2);
	CHECK(client.GetData() == first);
	CHECK(client.GetPacketQueueSize() == 1);
	CHECK(client.GetData() == second);
	CHECK(client.GetPacketQueueSize() == 0);
	CHECK_FALSE(client.HasIncomingData());

	client.SendData(first);
	CHECK(server.GetPacketQueueSize() == 1);
	server.Close(true);
	CHECK(server.GetPacketQueueSize() == 0);
}

TEST_CASE("Local connection counts the ping actually removed from its queue")
{
	netcode::CLocalConnection server;
	netcode::CLocalConnection client;
	auto& protocol = CBaseNetProtocol::Get();
	const auto frame = protocol.SendNewFrame();
	const auto ping = protocol.SendPing(0, 1, 0.0f);

	SECTION("Removing a non-front ping") {
		server.SendData(frame);
		server.SendData(ping);
		CHECK(client.GetNumQueuedPings() == 1);
		client.DeleteBufferPacketAt(1);
		CHECK(client.GetNumQueuedPings() == 0);
		CHECK(client.GetData() == frame);
		CHECK(client.GetNumQueuedPings() == 0);
	}
	SECTION("Removing a frame behind a ping") {
		server.SendData(ping);
		server.SendData(frame);
		CHECK(client.GetNumQueuedPings() == 1);
		client.DeleteBufferPacketAt(1);
		CHECK(client.GetNumQueuedPings() == 1);
		client.DeleteBufferPacketAt(99);
		CHECK(client.GetNumQueuedPings() == 1);
		CHECK(client.GetData() == ping);
		CHECK(client.GetNumQueuedPings() == 0);
	}
}

TEST_CASE("Flushing a local connection also clears its queued ping count")
{
	netcode::CLocalConnection server;
	netcode::CLocalConnection client;
	server.SendData(CBaseNetProtocol::Get().SendPing(0, 1, 0.0f));
	CHECK(client.GetNumQueuedPings() == 1);
	client.Close(false);
	CHECK(client.GetNumQueuedPings() == 1);
	CHECK(client.GetPacketQueueSize() == 1);
	client.Close(true);
	CHECK(client.GetNumQueuedPings() == 0);
	CHECK(client.GetPacketQueueSize() == 0);
	CHECK_FALSE(client.HasIncomingData());
}

TEST_CASE("Local ping counts can be observed while the server sends packets")
{
	netcode::CLocalConnection server;
	netcode::CLocalConnection client;
	const auto ping = CBaseNetProtocol::Get().SendPing(0, 1, 0.0f);
	constexpr unsigned int packets = 4096;
	std::atomic<bool> finished = false;
	std::thread sender([&]() {
		for (unsigned int i = 0; i < packets; ++i)
			server.SendData(ping);
		finished = true;
	});
	unsigned int received = 0;
	unsigned int peak = 0;
	while (!finished || client.HasIncomingData()) {
		peak = std::max(peak, client.GetNumQueuedPings());
		received += (client.GetData() != nullptr);
	}
	sender.join();
	CHECK(received == packets);
	CHECK(peak <= packets);
	CHECK(client.GetNumQueuedPings() == 0);
	CHECK(client.GetPacketQueueSize() == 0);
}
