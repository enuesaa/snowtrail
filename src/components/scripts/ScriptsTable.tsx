import { Table } from '@radix-ui/themes'
import { useListScripts } from '../../lib/scripts'
import { ScriptsTableItem } from './ScriptsTableItem'

export const ScriptsTable = () => {
  const { data: scripts } = useListScripts()

  return (
    <Table.Root m='5' mt='2'>
      <Table.Header>
        <Table.Row>
          <Table.ColumnHeaderCell>name</Table.ColumnHeaderCell>
          <Table.ColumnHeaderCell>command</Table.ColumnHeaderCell>
          <Table.ColumnHeaderCell>description</Table.ColumnHeaderCell>
          <Table.ColumnHeaderCell></Table.ColumnHeaderCell>
        </Table.Row>
      </Table.Header>
      <Table.Body>{scripts && scripts.map((v, i) => <ScriptsTableItem script={v} key={i} />)}</Table.Body>
    </Table.Root>
  )
}
