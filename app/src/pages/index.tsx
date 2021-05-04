import { Box } from "@chakra-ui/layout";
import Head from "next/head";
import { useEffect } from "react";
import Channels from "../components/chat/channels";
import ChatBox from "../components/chat/chatBox";
import MessageFeed from "../components/chat/messageFeed";
import useChatStore from "../hooks/store";

export default function Home() {
  const actions: any = useChatStore((state) => state.actions);

  // WIP will refactor this later, just sketching stuff out
  useEffect(() => {
    const ws = new WebSocket("ws://localhost:8080/api/v1/wsc");
    actions.setWS(ws);
    ws.onopen = (evt) => {
      console.log("connected", evt);
      ws.send(
        JSON.stringify({
          _type: "UserConnection",
          user: {
            id: 1,
            username: "kpo",
          },
        })
      );
    };

    ws.onmessage = (evt) => {
      const data = JSON.parse(evt.data);
      if (data?.data?.channels) {
        actions.setChannels(data.data.channels);
      }
      if (data?.data?.channel) {
        actions.addChannel(data.data.channel);
      }

      if (data?.data?.messages?.messages) {
        actions.setChannelMessages(data.data.messages);
      }

      if (data?.data?.message?.message) {
        actions.addMessage(data.data.message);
      }
      console.log("event", evt);
    };

    ws.onclose = (evt) => {
      console.log("disconnected", evt);
    };
  }, [actions]);
  
  return (
    <Box width="100%" height="100vh">
      <Head>
        <title>Reactionay Chatrs</title>
        <meta name="description" content="Messing around with react and rust" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <Box display="flex" width="100%" height="100%">
        <Channels />
        <Box display="flex" flexDirection="column" width="100%" height="100%">
          <MessageFeed />
          <ChatBox />
        </Box>
      </Box>
    </Box>
  );
}
