import { useProjectDeleteLazy, useProjectGetQuery } from '@/commands/poject'
import { PageTitle } from '@/components/common/PageTitle'
import { EventList } from '@/components/project/EventList'
import { ScriptList } from '@/components/project/ScriptList'
import { MouseEventHandler } from 'react'
import { useStyles } from '@/styles/use'

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

  const styles = useStyles((theme) => ({
    deleteBtn: theme({ surf: 'sub', size: 'x1', around: 'x2', decorate: 'rounded' }),
  }))


  return (
    <section>
      <PageTitle title={`Project ${project.name}`} />
      <ScriptList projectName={name} />
      <EventList />
      <button onClick={handleDeleteProject} css={styles.deleteBtn}>
        delete
      </button>
    </section>
  )
}
