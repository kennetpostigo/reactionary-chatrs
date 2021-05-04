import Icon from "@chakra-ui/icon";
import { Box, Heading, ListItem, Text, UnorderedList } from "@chakra-ui/layout";
import { useRouter } from "next/router";
import { FiHash } from "react-icons/fi";
import useChatStore from "../../hooks/store";
import Link from "../atoms/Link";

interface Message {
  id: string;
  content: string;
}

interface MessageFeedProps {
  feed?: Array<Message>;
}

const MessageFeed: React.FC<MessageFeedProps> = ({}) => {
  const { query } = useRouter();
  const feed = useChatStore((state) => state.messages);

  return (
    <Box
      display="flex"
      flexDirection="column"
      width="100%"
      flex="1"
      overflow="scroll"
    >
      <UnorderedList listStyleType="none" margin="0px" padding="0px">\
        {/* @ts-ignore */}
        {feed?.[query.channel]?.map((message) => (
          <ListItem
            key={message.id}
            height="50px"
            padding="10px"
            _hover={{
              transition: "0.25s background",
              bg: "whiteAlpha.50",
            }}
          >
            <Text fontWeight="600">{message.content}</Text>
          </ListItem>
        ))}
      </UnorderedList>
    </Box>
  );
};

export default MessageFeed;
