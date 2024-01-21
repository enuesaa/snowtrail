import { Button, Table } from '@radix-ui/themes';
import { Script, useRemoveScript } from '../../lib/scripts'
import { MouseEventHandler } from 'react';

type Props = {
  script: Script;
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
      <Table.Cell>
        <Button onClick={handleClick}>remove</Button>
      </Table.Cell>
    </Table.Row>
  )
}
