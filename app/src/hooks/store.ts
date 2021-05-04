import create from "zustand";

const useChatStore = create((set) => ({
  channels: [],
  users: [],
  messages: {},
  ws: undefined,
  actions: {
    setWS: (ws: any) => set((state: any) => ({ ws })),
    setChannels: (channels: any) => set((state) => ({ channels })),
    addChannel: (channel: any) =>
      set((state: any) => ({ channels: [...state.channels, channel] })),
    updateChannel: (channel: any) =>
      set((state: any) => {
        const cIdx = state.channels.find((c: any) => c.id === channel.id);
        if (cIdx === -1) return { channels: state.channels };
        state.channels[cIdx] = channel;
        return { channels: state.channels };
      }),
    deleteChannel: (id: any) =>
      set((state: any) => ({
        channels: state.channels.filter((c: any) => c.id === id),
      })),
    setChannelMessages: (msgs: any) =>
      set((state: any) => ({
        messages: { ...state.messages, [msgs.channel.name]: msgs.messages },
      })),
    addMessage: (msg: any) =>
      set((state: any) => {
        const channel = state.channels.find((c: any) => (c.id = msg.channel));
        if (!channel) return state;
        return {
          messages: {
            ...state.messages,
            [channel.name]: [...state.messages[channel.name], msg.message],
          },
        };
      }),
    updateMessage: (msg: any) =>
      set((state: any) => {
        const mIdx = state.messages[msg.channel].find(
          (m: any) => m.id === msg.id
        );
        if (!mIdx) return { messages: state.messages };
        state.messages[msg.channel][mIdx] = msg;
        return { messages: { ...state.messages } };
      }),
    deleteMessage: (msg: any) =>
      set((state: any) => ({
        messages: {
          ...state.messages,
          [msg.channel]: state.messages[msg.channel].filter(
            (m: any) => m.id === msg.id
          ),
        },
      })),
  },
}));

export default useChatStore;
