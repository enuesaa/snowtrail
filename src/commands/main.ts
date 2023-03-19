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

// events
export const {
  useAddEventQuery,
  useAddEventLazy,
} = useQueriesInit<{}, string>('addEvent')

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

// localfiles
export type LocalfileItem = {
  name: string;
}
export type LocalfilesResponse = {
  items: LocalfileItem[];
}
export type LocalfilesArg = {}
export const {
  useLocalfilesQuery,
  useLocalfilesLazy,
} = useQueriesInit<{}, LocalfilesResponse>('localfiles')

// surreal
export const {
  useStartSurrealQuery,
  useStartSurrealLazy,
} = useQueriesInit<{}, { status: 'string' }>('startSurreal')

export const {
  useEndSurrealQuery,
  useEndSurrealLazy,
} = useQueriesInit<{}, {}>('endSurreal')

// record
export const {
  useRecordQuery,
  useRecordLazy,
} = useQueriesInit<{}, {}>('record')
