import { Table } from '@radix-ui/themes'
import { Link } from '@/components/common/Link'

type Props = {
  name: string
}
export const LogsTableItem = ({ name }: Props) => {
  return (
    <Table.Row>
      <Table.Cell>
        <Link href={`/logs/${name}`}>{name}</Link>
      </Table.Cell>
    </Table.Row>
  )
}
