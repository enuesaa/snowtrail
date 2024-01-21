import { Table } from '@radix-ui/themes'
import { useListScripts } from '../../lib/scripts'

export const ScriptsTable = () => {
  const {data: scripts} = useListScripts()

  return (
    <Table.Root>
      <Table.Header>
        <Table.Row>
          <Table.ColumnHeaderCell>name</Table.ColumnHeaderCell>
        </Table.Row>
      </Table.Header>
      <Table.Body>
        {scripts && scripts.map((v, i) => (
          <Table.Row key={i}>
            <Table.Cell>{v.name}</Table.Cell>
          </Table.Row>
        ))}
      </Table.Body>
    </Table.Root>
  )
}
