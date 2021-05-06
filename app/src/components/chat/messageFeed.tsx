import { Box, ListItem, Text, UnorderedList } from "@chakra-ui/layout";
import useChatStore from "../../hooks/useChatStore";

interface MessageFeedProps {}

const MessageFeed: React.FC<MessageFeedProps> = ({}) => {
  const me = useChatStore((state) => state.me as string);
  const activeChannel = useChatStore((state) => state.channel);
  const feed = useChatStore((state) => state.messages);

  return (
    <Box
      overflow="auto"
      display="flex"
      flexDirection="column-reverse"
      flex="1"
      width="100%"
    >
      <UnorderedList listStyleType="none" margin="0px" padding="20px 10px">
        {feed?.[activeChannel]?.map((message) => (
          <ListItem
            display="flex"
            flexDirection="column"
            justifyContent="center"
            key={message.id}
            height="55px"
            padding="10px"
            borderRadius="0.375rem"
            _hover={{
              transition: "0.25s background",
              bg: "whiteAlpha.50",
            }}
          >
            <Text
              margin="0px"
              padding="0px"
              fontWeight="bold"
              color={message.username === me ? "green.600" : "blue.600"}
            >
              {message.username}
            </Text>
            <Text margin="0px" padding="0px" fontWeight="500">
              {message.content}
            </Text>
          </ListItem>
        ))}
      </UnorderedList>
    </Box>
  );
};

export default MessageFeed;
