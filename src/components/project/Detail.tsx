import { useProjectDeleteLazy, useProjectGetQuery } from '@/commands/poject'
import { PageTitle } from '@/components/common/PageTitle'
import { EventList } from '@/components/project/EventList'
import { ScriptList } from '@/components/project/ScriptList'
import { MouseEventHandler } from 'react'

type Props = {
  name: string
}
export const Detail = ({ name }: Props) => {
  const project = useProjectGetQuery({ name })
  const { invoke: invokeDeleteProject } = useProjectDeleteLazy()
  if (project === null) {
    return <></>
  }
  const handleDeleteProject: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invokeDeleteProject({ name })
  }

  return (
    <section>
      <PageTitle title={project.name} />
      <ScriptList name={name} />
      <EventList />
      <button onClick={handleDeleteProject}>delete</button>
    </section>
  )
}
