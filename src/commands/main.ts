import { useQueriesInit } from '@/commands/use'

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
} = useQueriesInit<{}, GitHistoriesResponse>('gitHistories')

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
} = useQueriesInit<{ url: string }, MessageResponse>('feed')

// greet
export const {
  useGreetQuery,
  useGreetLazy,
} = useQueriesInit<{ name: string }, string>('greet')

// record
export const {
  useRecordQuery,
  useRecordLazy,
} = useQueriesInit<{}, {}>('record')
