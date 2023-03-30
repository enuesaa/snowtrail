import { PageTitle } from '@/components/common/PageTitle'
import { ProjectEvents } from '@/components/project/ProjectEvents'
import { ScriptBoard } from '@/components/script/ScriptBoard'

export const ProjectDetail = () => {
  return (
    <section>
      <PageTitle title='Project aa' />
      <ScriptBoard />
      <ProjectEvents />
    </section>
  )
}