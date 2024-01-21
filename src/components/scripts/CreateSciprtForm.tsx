import { Button, Dialog, Flex, Text, TextField } from '@radix-ui/themes'
import { Script, useAddScript } from '../../lib/scripts'
import { useForm } from 'react-hook-form'

export const CreateScriptForm = () => {
  const addScript = useAddScript()
  const form = useForm<Script>()

  const hanldeSubmit = form.handleSubmit(async (data) => {
    console.log('a', data)
    await addScript.mutateAsync(data)
    form.reset()
  })

  return (
    <Dialog.Root>
      <Dialog.Trigger>
        <Button>Create</Button>
      </Dialog.Trigger>

      <Dialog.Content style={{ maxWidth: 450 }}>
        <Dialog.Title>Create new Script</Dialog.Title>
        <form onSubmit={hanldeSubmit}>
          <label>
            <Text as='div' size='2' mb='1' weight='bold'>Name</Text>
            <TextField.Input data-1p-ignore {...form.register('name')} />
          </label>

          <Flex gap='3' justify='end'>
            <Dialog.Close>
              <Button variant='soft' color='gray'>Cancel</Button>
            </Dialog.Close>
            <Dialog.Close>
              <Button type='submit'>Save</Button>
            </Dialog.Close>
          </Flex>
        </form>
      </Dialog.Content>
    </Dialog.Root>
  )
}
