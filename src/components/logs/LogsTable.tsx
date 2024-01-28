import { Table } from '@radix-ui/themes'
import { LogsTableItem } from './LogsTableItem'
import { useListLogs } from '@/lib/logs'

export const LogsTable = () => {
  const { data: logs } = useListLogs()

  return (
    <Table.Root m='5' mt='2'>
      <Table.Header>
        <Table.Row>
          <Table.ColumnHeaderCell>name</Table.ColumnHeaderCell>
          <Table.ColumnHeaderCell></Table.ColumnHeaderCell>
        </Table.Row>
      </Table.Header>
      <Table.Body>{logs && logs.map((v, i) => <LogsTableItem log={v} key={i} />)}</Table.Body>
    </Table.Root>
  )
}
