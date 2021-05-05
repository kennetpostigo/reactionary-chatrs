import { Box, Heading, ListItem, Text, UnorderedList } from "@chakra-ui/layout";
import { Textarea } from "@chakra-ui/textarea";
import { useState } from "react";
import useChatStore from "../../hooks/useChatStore";

interface ChatBoxProps {}

const ChatBox: React.FC<ChatBoxProps> = ({}) => {
  const ws: any = useChatStore((state) => state.ws);
  const me = useChatStore((state) => state.me);
  const activeChannel = useChatStore((state) => state.channel);
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
                    username: me,
                    content: v,
                    channel_id: activeChannel,
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
