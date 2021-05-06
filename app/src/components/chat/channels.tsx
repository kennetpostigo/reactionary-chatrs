import { Button } from "@chakra-ui/button";
import Icon from "@chakra-ui/icon";
import { Input } from "@chakra-ui/input";
import { Box, Heading, ListItem, Text, UnorderedList } from "@chakra-ui/layout";
import { useEffect, useState } from "react";
import { FiHash } from "react-icons/fi";
import useChatStore from "../../hooks/useChatStore";
import Link from "../atoms/Link";

interface ChannelsProps {}

const Channels: React.FC<ChannelsProps> = ({}) => {
  const [newChannel, setNewChannel] = useState(() => "");
  const [username, setUsername] = useState(() => "");
  const [identified, setIdentified] = useState(() => false);
  const actions = useChatStore((state) => state.actions);
  const ws = useChatStore((state) => state.ws);
  const activeChannel = useChatStore((state) => state.channel);
  const channels = useChatStore((state) => state.channels);

  useEffect(() => {
    if (channels.length && !activeChannel) {
      console.log("channels[0].id", channels[0].id);
      actions.setChannel(channels[0].id);
      ws.send(
        JSON.stringify({
          _type: "RetrieveChannelMessages",
          channel: channels[0],
        })
      );
    }
  }, [actions, channels, activeChannel]);

  return (
    <Box
      display="flex"
      flexDirection="column"
      width="280px"
      height="100%"
      bg="blackAlpha.300"
      padding="10px"
    >
      <Box flex={1}>
        {!identified && (
          <>
            <Heading fontSize="20px" mb="0px" pb="0px">
              Set Username
            </Heading>
            <Input
              mt="5px"
              placeholder="username"
              value={username}
              onChange={(e) => setUsername(e.target.value)}
            />
            <Button
              width="100%"
              mt="5px"
              onClick={(_e) => {
                setIdentified(() => true);
                actions.setMe(username);
                ws.send(
                  JSON.stringify({
                    _type: "UserConnection",
                    user: {
                      id: username.length,
                      username: username,
                    },
                  })
                );
              }}
            >
              Connect
            </Button>
          </>
        )}
        {!channels.length ? null : (
          <>
            <Heading>Channels</Heading>
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
          </>
        )}
      </Box>
      {identified && (
        <Box
          display="flex"
          flexDirection="column"
          alignSelf="center"
          width="90%"
          mb="10px"
        >
          <Heading fontSize="20px" mb="0px" pb="0px">
            New Channel
          </Heading>
          <Input
            mt="5px"
            placeholder="channel name"
            value={newChannel}
            onChange={(e) => setNewChannel(e.target.value)}
          />
          <Button
            mt="5px"
            onClick={(_e) => {
              ws.send(
                JSON.stringify({
                  _type: "NewChannel",
                  channel: { name: newChannel },
                })
              );
            }}
          >
            Add Channel
          </Button>
        </Box>
      )}
    </Box>
  );
};

export default Channels;
