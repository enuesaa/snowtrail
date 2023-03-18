import { useQueriesInit } from '@/commands/use'

export type GitHistory = {
  hash: string;
}
export type GitHistoriesResponse = {
  items: GitHistory[];
}
export type GitHistoriesArg = {}

export const {
  useGitHistoriesQuery,
  useGitHistoriesLazy,
} = useQueriesInit<GitHistoriesArg, GitHistoriesResponse>('gitHistories')