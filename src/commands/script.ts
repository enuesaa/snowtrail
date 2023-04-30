import { queriesInit } from '@/commands/use'

export type Script = {
  id?: string | null;
  name: string;
  commands: string[];
  project_id: string;
}

export const { useScriptListQuery, useScriptListLazy } = queriesInit<{ projectId: string }, Script[]>('script_list')

export const { useScriptGetQuery, useScriptGetLazy } = queriesInit<{ id: string }, Script>('script_get')

export const { useScriptCreateQuery, useScriptCreateLazy } = queriesInit<{ data: Script }, void>('script_create')

export const { useScriptDeleteQuery, useScriptDeleteLazy } = queriesInit<{ id: string }, void>('script_delete')

export const {
  useScriptRunQuery,
  useScriptRunLazy,
} = queriesInit<{ id: string }, void>('script_run')
