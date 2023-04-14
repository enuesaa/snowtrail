import { useStyles } from '@/styles/use'
import Link from 'next/link'

type Props = {
  title: string
  id: string
}
export const SubscribeBoardItem = ({ title, id }: Props) => {
  const styles = useStyles((theme) => ({
    card: theme(),
  }))

  return (
    <Link href={`/subscribe/items/${id}`} css={styles.card}>
      {title}
    </Link>
  )
}
