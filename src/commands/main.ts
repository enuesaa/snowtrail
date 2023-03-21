import { queriesInit } from '@/commands/use'

// git histories
export type GitHistory = {
  hash: string;
}
export type GitHistoriesResponse = {
  items: GitHistory[];
}
export const {
  useGitHistoriesQuery,
  useGitHistoriesLazy,
} = queriesInit<{}, GitHistoriesResponse>('gitHistories')

// feed
type MessageItem = {
  title: string;
  url: string;
  descripion: string;
}
export type MessageResponse = {
  title: string;
  items: MessageItem[];
}
export const {
  useFeedQuery,
  useFeedLazy,
} = queriesInit<{ url: string }, MessageResponse>('feed')

// greet
export const {
  useGreetQuery,
  useGreetLazy,
} = queriesInit<{ name: string }, string>('greet')

// record
export const {
  useRecordQuery,
  useRecordLazy,
} = queriesInit<{}, {}>('record')
