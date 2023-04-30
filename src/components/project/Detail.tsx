import { useProjectDeleteLazy, useProjectGetQuery } from '@/commands/poject'
import { PageTitle } from '@/components/common/PageTitle'
import { ScriptList } from '@/components/project/ScriptList'
import { useStyles } from '@/styles/use'
import { MouseEventHandler } from 'react'
import { Dashboard as SubscribeDashboard } from '@/components/subscribe/Dashboard'

type Props = {
  id: string
}
export const Detail = ({ id }: Props) => {
  const project = useProjectGetQuery({ id })
  const { invoke: invokeDeleteProject } = useProjectDeleteLazy()
  const styles = useStyles((theme) => ({
    deleteBtn: theme({ surf: 'sub', size: 'x1', around: 'x2', decorate: 'rounded' }),
  }))
  if (project === null) {
    return <></>
  }
  const handleDeleteProject: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    invokeDeleteProject({ id })
  }

  return (
    <section>
      <PageTitle title={`${project.name}`} />
      Detail: NextJS App
      <ScriptList projectId={id} />
      <SubscribeDashboard />
      <button onClick={handleDeleteProject} css={styles.deleteBtn}>
        delete
      </button>
    </section>
  )
}
