import { PageTitle } from '@/components/common/PageTitle'
import { ProjectEvents } from '@/components/project/ProjectEvents'

export const ProjectDetail = () => {
  return (
    <section>
      <PageTitle title='Project aa' />
      Subscribes
      <ProjectEvents />
    </section>
  )
}