import { atom } from 'jotai'
import { type Fetchmeta } from '@/states/type'

export type GitHistory = {
  hash: string;
}
export type GitHistoriesResponse = {
  items: GitHistory[];
}
export type GitHistoriesArg = {}

export const gitHistoriesMeta: Fetchmeta<GitHistoriesResponse, GitHistoriesArg> = {
  name: 'git_histories',
  atom: atom<GitHistoriesResponse | null>(null),
}
