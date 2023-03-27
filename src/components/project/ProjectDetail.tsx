import { PageTitle } from '@/components/common/PageTitle'
import { css, useTheme } from '@emotion/react'
import { ProjectEvents } from '@/components/project/ProjectEvents'
import { ScriptBoard } from '@/components/script/ScriptBoard'

export const ProjectDetail = () => {
  const theme = useTheme()

  const styles = {
    main: css({
      margin: '20px',
      padding: '0 10px 10px 10px',
      color: theme.color.main,
    }),
  }

  return (
    <section css={styles.main}>
      <PageTitle title='Project aa' />
      <ScriptBoard />
      <ProjectEvents />
    </section>
  )
}