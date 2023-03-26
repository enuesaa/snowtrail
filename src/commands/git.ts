import { queriesInit } from '@/commands/use'

export type GitHistory = {
  hash: string;
}
export type GitHistoriesResponse = {
  items: GitHistory[];
}
export const {
  useGitHistoriesQuery,
  useGitHistoriesLazy,
} = queriesInit<{}, GitHistoriesResponse>('git_histories')

export const {
  usePushGitHistoriesToEventQuery,
  usePushGitHistoriesToEventLazy,
} = queriesInit<{}, {}>('push_git_histories_to_event')
