import { atom } from 'jotai'
import { type Fetchmeta } from './type'

export type LocalfileItem = {
  name: string;
}
export type LocalfilesResponse = {
  items: LocalfileItem[];
}
export type LocalfilesArg = {}

export const localfilesMeta: Fetchmeta<LocalfilesResponse, LocalfilesArg> = {
  name: 'localfiles',
  atom: atom<LocalfilesResponse | null>(null),
}
