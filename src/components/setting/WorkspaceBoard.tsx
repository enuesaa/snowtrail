import { useGetWorkspaceQuery } from '@/commands/workspace'
import { PageTitle } from '@/components/common/PageTitle'
import { css, useTheme } from '@emotion/react'

export const WorkspaceBoard = () => {
  const theme = useTheme()
  const data = useGetWorkspaceQuery({})
  console.log(data)

  const styles = {
    main: css({
      margin: '20px',
      padding: '0 10px 10px 10px',
      color: theme.color.main,
    }),
    form: css({
      'input': theme.input,
      'button': theme.input,
    }),
  }

  return (
    <section css={styles.main}>
      <PageTitle title='workspace' />
      <form css={styles.form}>
        <input type='text' defaultValue={'dir'} />
        <input type='text' defaultValue={'auto_add_new_project'} />
        <button>save</button>
      </form>
    </section>
  )
}