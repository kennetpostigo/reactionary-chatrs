import create from "zustand";

const useChatStore = create((set) => ({
  me: "",
  channels: [],
  users: [],
  messages: {},
  ws: undefined,
  channel: undefined,
  actions: {
    setMe: (me) => set((state) => ({ ...state, me })),
    setWS: (ws: any) => set((state: any) => ({ ...state, ws })),
    setChannel: (channel) => set((state) => ({ channel })),
    setChannels: (channels: any) =>
      set((state: any) => {
        const next = { ...state, channels };
        return next;
      }),
    addChannel: (channel: any) =>
      set((state: any) => {
        const next = {
          ...state,
          channels: [...state.channels, channel],
        };
        return next;
      }),
    updateChannel: (channel: any) =>
      set((state: any) => {
        const cIdx = state.channels.find((c: any) => c.id === channel.id);
        if (cIdx === -1) return { channels: state.channels };
        state.channels[cIdx] = channel;
        return { ...state, channels: state.channels };
      }),
    deleteChannel: (id: any) =>
      set((state: any) => ({
        ...state,
        channels: state.channels.filter((c: any) => c.id === id),
      })),
    setChannelMessages: (msgs: any) =>
      set((state: any) => {
        const next = {
          ...state,
          messages: { ...state.messages, [msgs.channel.id]: msgs.messages },
        };
        return next;
      }),
    addMessage: (msg: any) =>
      set((state: any) => {
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
    updateMessage: (msg: any) =>
      set((state: any) => {
        const mIdx = state.messages[msg.channel].find(
          (m: any) => m.id === msg.id
        );
        if (!mIdx) return state;
        state.messages[msg.channel][mIdx] = msg;
        return { ...state, messages: { ...state.messages } };
      }),
    deleteMessage: (msg: any) =>
      set((state: any) => ({
        ...state,
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
