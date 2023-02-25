import { atom } from 'jotai'
import { type FetchAtom } from './type'

type MessageItem = {
  title: string;
  url: string;
  descripion: string;
}
export type MessageResponse = {
  title: string;
  items: MessageItem[];
}
export type MessageArg = {
  url: string;
}

export const messageAtom = atom<FetchAtom<MessageResponse, MessageArg>>({ name: 'feed', res: null, arg: { url: '' }})

