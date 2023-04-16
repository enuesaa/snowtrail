import { PageTitle } from '@/components/common/PageTitle'
import { ProjectEvents } from '@/components/project/ProjectEvents'
import { ProjectScripts } from '@/components/project/PorjectScripts'
import { useProjectGetQuery } from '@/commands/poject'
import { useProjectDeleteLazy } from '@/commands/poject'
import { MouseEventHandler } from 'react'

type Props = {
  name: string;
}
export const ProjectDetail = ({ name }: Props) => {
  const project = useProjectGetQuery({ name })
  const { invoke: invokeDeleteProject } = useProjectDeleteLazy()
  if (project === null) {
    return (<></>)
  }
  const handleDeleteProject: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invokeDeleteProject({ name })
  }

  return (
    <section>
      <PageTitle title={project.name} />
      <ProjectScripts />
      <ProjectEvents />
      <button onClick={handleDeleteProject}>delete</button>
    </section>
  )
}
