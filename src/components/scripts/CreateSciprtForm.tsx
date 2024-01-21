import { Button, Dialog, Flex, Text, TextField } from '@radix-ui/themes'
import { ScriptSchema, useAddScript } from '../../lib/scripts'
import { useForm } from 'react-hook-form'

export const CreateScriptForm = () => {
  const addScript = useAddScript()
  const form = useForm<ScriptSchema>()

  const hanldeSubmit = form.handleSubmit(async (data) => {
    await addScript.mutateAsync(data)
    form.reset()
  })

  return (
    <Dialog.Root>
      <Dialog.Trigger>
        <Button m='2' style={{ cursor: 'pointer' }}>Create</Button>
      </Dialog.Trigger>

      <Dialog.Content style={{ maxWidth: 450 }}>
        <Dialog.Title>Create new Script</Dialog.Title>
        <form onSubmit={hanldeSubmit}>
        <label>
            <Text as='div' size='2' mb='1' weight='bold'>Name</Text>
            <TextField.Input data-1p-ignore {...form.register('name')} />
          </label>
          <label>
            <Text as='div' size='2' mb='1' weight='bold'>Command</Text>
            <TextField.Input data-1p-ignore {...form.register('command')} />
          </label>
          <label>
            <Text as='div' size='2' mb='1' weight='bold'>Description</Text>
            <TextField.Input data-1p-ignore {...form.register('description')} />
          </label>

          <Flex gap='3' justify='end' mt='5'>
            <Dialog.Close>
              <Button variant='soft' color='gray' style={{ cursor: 'pointer' }}>Cancel</Button>
            </Dialog.Close>
            <Dialog.Close>
              <Button type='submit' style={{ cursor: 'pointer' }}>Save</Button>
            </Dialog.Close>
          </Flex>
        </form>
      </Dialog.Content>
    </Dialog.Root>
  )
}
