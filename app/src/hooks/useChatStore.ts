import create from "zustand";

interface Channel {
  id: string;
  name: string;
}

interface Message {
  id: string;
  content: string;
  username: string;
  channel_id: string;
}

interface User {
  id: number;
  username: string;
}

export type ChatStore = {
  me: string;
  channels: Array<Channel>;
  messages: { [key: string]: Array<Message> };
  users: Array<User>;
  ws: WebSocket;
  channel: string;
  actions: {
    setMe: (me: string) => void;
    setWS: (ws: WebSocket) => void;
    setChannel: (channel: string) => void;
    setChannels: (channels: Array<Channel>) => void;
    addChannel: (channel: Channel) => void;
    updateChannel: (channel: Channel) => void;
    deleteChannel: (id: string) => void;
    setChannelMessages: (channelMessages: {
      channel: Channel;
      messages: Array<Message>;
    }) => void;
    addMessage: (msg: { channel: string; message: Message }) => void;
    updateMessage: (msg: Message) => void;
  };
}

const useChatStore = create<ChatStore>((set) => ({
  me: "",
  channels: [],
  users: [],
  messages: {},
  ws: undefined,
  channel: undefined,
  actions: {
    setMe: (me) => set((state) => ({ ...state, me })),
    setWS: (ws) => set((state) => ({ ...state, ws })),
    setChannel: (channel) => set((state) => ({ ...state, channel })),
    setChannels: (channels) =>
      set((state) => {
        const next = { ...state, channels };
        return next;
      }),
    addChannel: (channel) =>
      set((state) => {
        const next = {
          ...state,
          channels: [...state.channels, channel],
        };
        return next;
      }),
    updateChannel: (channel) =>
      set((state) => {
        const cIdx = state.channels.findIndex((c) => c.id === channel.id);
        if (cIdx === -1) return { channels: state.channels };
        state.channels[cIdx] = channel;
        return { ...state, channels: state.channels };
      }),
    deleteChannel: (id) =>
      set((state) => ({
        ...state,
        channels: state.channels.filter((c) => c.id === id),
      })),
    setChannelMessages: (msgs) =>
      set((state) => {
        const next = {
          ...state,
          messages: { ...state.messages, [msgs.channel.id]: msgs.messages },
        };
        return next;
      }),
    addMessage: (msg) =>
      set((state) => {
        const next = {
          ...state,
          messages: {
            ...state.messages,
            [msg.channel]: [
              ...(state.messages[msg.channel] || []),
              msg.message,
            ],
          },
        };
        return next;
      }),
    updateMessage: (msg) =>
      set((state) => {
        const mIdx = state.messages[msg.channel_id].findIndex(
          (m) => m.id === msg.id
        );
        if (mIdx !== -1) return state;
        state.messages[msg.channel_id][mIdx] = msg;
        return { ...state, messages: { ...state.messages } };
      }),
  },
}));

export default useChatStore;
