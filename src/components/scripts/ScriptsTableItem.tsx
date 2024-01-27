import { Button, Table } from '@radix-ui/themes'
import { ScriptSchema, useRemoveScript } from '@/lib/scripts'
import { MouseEventHandler } from 'react'

type Props = {
  script: ScriptSchema
}
export const ScriptsTableItem = ({ script }: Props) => {
  const removeScript = useRemoveScript()
  const handleClick: MouseEventHandler<HTMLButtonElement> = async (e) => {
    e.preventDefault()
    await removeScript.mutateAsync(script.name)
  }

  return (
    <Table.Row>
      <Table.Cell>{script.name}</Table.Cell>
      <Table.Cell>{script.command}</Table.Cell>
      <Table.Cell>{script.description}</Table.Cell>
      <Table.Cell>
        <Button onClick={handleClick} style={{ cursor: 'pointer' }}>
          remove
        </Button>
      </Table.Cell>
    </Table.Row>
  )
}
