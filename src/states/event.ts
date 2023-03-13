import { atom } from 'jotai'
import { type Fetchmeta } from '@/states/type'

export type EventResponse = string;
export type EventArg = {}

export const eventMeta: Fetchmeta<EventResponse, EventArg> = {
  name: 'add_event',
  atom: atom<EventResponse | null>(null),
}
