import { atom } from 'jotai'
import { type Fetchmeta } from '@/states/type'

export type StartSurrealResponse = {
  status: string;
}
export type StartSurrealArg = {}

export const startSurrealMeta: Fetchmeta<StartSurrealResponse, StartSurrealArg> = {
  name: 'start_surreal',
  atom: atom<StartSurrealResponse | null>(null),
}


export type EndSurrealResponse = {}
export type EndSurrealArg = {}

export const endSurrealMeta: Fetchmeta<EndSurrealResponse, EndSurrealArg> = {
  name: 'end_surreal',
  atom: atom<EndSurrealResponse | null>(null),
}


export const recordMeta: Fetchmeta<{}, {}> = {
  name: 'record',
  atom: atom<{} | null>(null),
}
