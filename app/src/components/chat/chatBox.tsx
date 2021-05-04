import Icon from "@chakra-ui/icon";
import { Input } from "@chakra-ui/input";
import { Box, Heading, ListItem, Text, UnorderedList } from "@chakra-ui/layout";
import { Textarea } from "@chakra-ui/textarea";
import { useRouter } from "next/router";
import { useState } from "react";
import useChatStore from "../../hooks/store";

interface ChatBoxProps {}

const ChatBox: React.FC<ChatBoxProps> = ({}) => {
  const { query } = useRouter();
  const ws: any = useChatStore((state) => state.ws);
  const channels: any = useChatStore((state) => state.channels);
  const [v, setV] = useState(() => "");
  return (
    <Box
      display="flex"
      width="100%"
      minHeight="80px"
      pl="20px"
      pr="20px"
      pb="20px"
    >
      <Box
        width="100%"
        minHeight="50px"
        maxHeight="300px"
        borderRadius="0.375rem"
        bg="whiteAlpha.100"
      >
        <Textarea
          value={v}
          onChange={(e) => setV(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Enter") {
              ws.send(
                JSON.stringify({
                  _type: "NewMessage",
                  message: {
                    username: "kpo",
                    content: v,
                    channel_id: channels.find((c) => c.name === query.channel)
                      .id,
                  },
                })
              );
              setV("");
            }
          }}
          bg="transparent"
          border="none"
          width="100%"
          height="100%"
          resize="none"
          borderColor="whiteAlpha.100"
          borderRadius="0.375rem"
          _focus={{ outline: "transparent" }}
        />
      </Box>
    </Box>
  );
};

export default ChatBox;
