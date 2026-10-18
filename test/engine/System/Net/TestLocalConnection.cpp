/* This file is part of the Spring engine (GPL v2 or later), see LICENSE.html */

#include "System/Net/LocalConnection.h"
#include "Net/Protocol/BaseNetProtocol.h"

#include <catch_amalgamated.hpp>

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
