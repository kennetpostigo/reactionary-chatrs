import { Button } from "@chakra-ui/button";
import Icon from "@chakra-ui/icon";
import { Input } from "@chakra-ui/input";
import { Box, Heading, ListItem, Text, UnorderedList } from "@chakra-ui/layout";
import { useState } from "react";
import { FiHash } from "react-icons/fi";
import useChatStore from "../../hooks/useChatStore";
import Link from "../atoms/Link";

interface Channel {
  id: string;
  name: string;
}

interface ChannelsProps {
  channels?: Array<Channel>;
}

const Channels: React.FC<ChannelsProps> = ({}) => {
  const [c, setC] = useState(() => "");
  const [u, setU] = useState(() => "");
  const actions: any = useChatStore((state) => state.actions);
  const ws: any = useChatStore((state) => state.ws);
  const channels: any = useChatStore((state) => state.channels);

  return (
    <Box
      display="flex"
      flexDirection="column"
      width="280px"
      height="100%"
      bg="blackAlpha.300"
      padding="10px"
    >
      {
        <>
          <Text as="h5" mb="0px" pb="0px">
            Set your username
          </Text>
          <Input
            placeholder="username"
            value={u}
            onChange={(e) => setU(e.target.value)}
          />
          <Button
            onClick={(_e) => {
              ws.send(
                JSON.stringify({
                  _type: "UserConnection",
                  user: {
                    id: u.length,
                    username: u,
                  },
                })
              );
            }}
          >
            Connect
          </Button>
        </>
      }
      <Input
        placeholder="channel name"
        value={c}
        onChange={(e) => setC(e.target.value)}
      />
      <Button
        onClick={(_e) => {
          ws.send(
            JSON.stringify({ _type: "NewChannel", channel: { name: c } })
          );
        }}
      >
        Add Channel
      </Button>
      <UnorderedList listStyleType="none" margin="0px" padding="0px">
        {channels?.map((channel) => (
          <ListItem
            key={channel.name}
            height="30px"
            mt="5px"
            mb="5px"
            pl="10px"
            pr="10px"
            borderRadius="0.375rem"
            _hover={{
              transition: "0.3s background",
              bg: "whiteAlpha.200",
            }}
          >
            <Link
              display="flex"
              alignItems="center"
              href={`/?channel=${channel.name}`}
              onClick={() => {
                actions.setChannel(channel.id);
                ws.send(
                  JSON.stringify({
                    _type: "RetrieveChannelMessages",
                    channel,
                  })
                );
              }}
            >
              <Icon as={FiHash} mr="0.5rem" mt="4px" fontSize="lg" />
              <Text fontWeight="600" padding="0px" mt="2px">
                {channel.name}
              </Text>
            </Link>
          </ListItem>
        ))}
      </UnorderedList>
    </Box>
  );
};

export default Channels;
