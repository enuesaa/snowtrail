import { atom } from 'jotai'
import { type Fetchmeta } from './type'

export type RunlsResponse = string
export type RunlsArg = {}

export const runlsMeta: Fetchmeta<RunlsResponse, RunlsArg> = {
  name: 'runls',
  atom: atom<RunlsResponse | null>(null),
}
