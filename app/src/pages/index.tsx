import { Box } from "@chakra-ui/layout";
import Head from "next/head";
import Channels from "../components/chat/channels";
import ChatBox from "../components/chat/chatBox";
import MessageFeed from "../components/chat/messageFeed";
import useChatSocket from "../hooks/../hooks/useChatSocket";

export default function Home() {
  useChatSocket();
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
