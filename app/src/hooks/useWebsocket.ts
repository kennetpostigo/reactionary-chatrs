import { useEffect } from "react";
import useChatStore from "./useChatStore";

const useWebsocket = () => {
  const actions: any = useChatStore((state) => state.actions);

  useEffect(() => {
    const ws = new WebSocket("ws://localhost:8080/api/v1/wsc");
    actions.setWS(ws);
    ws.onopen = (evt) => {
      console.log("connected", evt);
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
      console.log("event", data);
    };

    ws.onclose = (evt) => {
      console.log("disconnected", evt);
    };
  }, [actions]);
};

export default useWebsocket;