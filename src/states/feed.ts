import { atom } from 'jotai'
import { type Fetchmeta } from './type'

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

export const messageMeta: Fetchmeta<MessageResponse, MessageArg> = {
  name: 'feed',
  atom: atom<MessageResponse | null>(null),
}
