import { useGetWorkspaceQuery } from '@/commands/workspace'
import { PageTitle } from '@/components/common/PageTitle'
import { css, useTheme } from '@emotion/react'

export const WorkspaceBoard = () => {
  const theme = useTheme()
  const data = useGetWorkspaceQuery({})
  console.log(data)

  const styles = {
    form: css({
      'input': theme.input,
      'button': theme.input,
    }),
  }

  return (
    <section>
      <PageTitle title='workspace' />
      <form css={styles.form}>
        <input type='text' defaultValue={'dir'} />
        <input type='text' defaultValue={'auto_add_new_project'} />
        <button>save</button>
      </form>
    </section>
  )
}