import { Table } from '@radix-ui/themes'
import { type LogSchema } from '@/lib/logs'
import { Link } from '@/components/common/Link'

type Props = {
  log: LogSchema
}
export const LogsTableItem = ({ log }: Props) => {
  return (
    <Table.Row>
      <Table.Cell>{log.name}</Table.Cell>
      <Table.Cell>
        <Link href={`/logs/${log.name}`}>Go</Link>
      </Table.Cell>
    </Table.Row>
  )
}