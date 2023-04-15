import { queriesInit } from '@/commands/use'

export type Project = {
  name: string;
  workdir: string;
}
export const {
  useProjectListQuery,
  useProjectListLazy
} = queriesInit<void, Project[]>('project_list')

export const {
  useProjectGetQuery,
  useProjectGetLazy
} = queriesInit<{ name: string }, Project>('project_get')

export const {
  useProjectCreateQuery,
  useProjectCreateLazy
} = queriesInit<{ data: Project }, {}>('project_create')

export const {
  useProjectDeleteQuery,
  useProjectDeleteLazy
} = queriesInit<{ name: string }, Project>('project_delete')
