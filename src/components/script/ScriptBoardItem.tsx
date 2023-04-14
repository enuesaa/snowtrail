import { useStyles } from '@/styles/use'
import Link from 'next/link'

type Props = {
  title: string;
  id: string;
}
export const ScriptBoardItem = ({ title, id }: Props) => {
  

  const styles = useStyles(theme => ({
    card: theme(),
  }))

  return (
    <Link href={`/script/items/${id}`} css={styles.card}>
      {title}
    </Link>
  )
}