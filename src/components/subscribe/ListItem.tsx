import { useStyles } from '@/styles/use'
import Link from 'next/link'

type Props = {
  title: string
  id: string
}
export const ListItem = ({ title, id }: Props) => {
  const styles = useStyles((theme) => ({
    card: theme(),
  }))

  return (
    <Link href={`/subscribes/${id}`} css={styles.card}>
      {title}
    </Link>
  )
}
