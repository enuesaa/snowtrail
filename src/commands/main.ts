import { useLazyQueryInit, useQueryInit } from '@/commands/use'

export type GitHistory = {
  hash: string;
}
export type GitHistoriesResponse = {
  items: GitHistory[];
}
export type GitHistoriesArg = {}

export const useGitHistoriesQuery = useQueryInit<GitHistoriesArg, GitHistoriesResponse>('git_histories')
export const useGitHistoriesLazyQuery = useLazyQueryInit<GitHistoriesArg, GitHistoriesResponse>('git_histories')
