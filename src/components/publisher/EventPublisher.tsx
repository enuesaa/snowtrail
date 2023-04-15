import { useStyles } from '@/styles/use'
import { PageTitle } from '@/components/common/PageTitle'
import { useEventPublishLazy } from '@/commands/event'
import { useState, MouseEventHandler } from 'react'
import { FaPlus, FaMinus } from 'react-icons/fa'
import { nanoid } from 'nanoid'
import { useForm } from 'react-hook-form'
import { TextInput } from '@/components/common/TextInput'

type FormDataKv = {
  name: string
  value: string
}
type FormData = {
  name: string
  kvs: FormDataKv[]
}
export const EventPublisher = () => {
  const [valueIds, setValueIds] = useState<string[]>([])
  const { invoke } = useEventPublishLazy()
  const { register, handleSubmit } = useForm<FormData>()

  const handleAddValue: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    setValueIds([...valueIds, nanoid()])
  }
  const removeValue = (id: string) => {
    setValueIds(valueIds.filter((v) => v !== id))
  }
  const handlePublish = handleSubmit((data) => {
    if (data.kvs === undefined) {
      data.kvs = []
    }
    invoke({ data })
  })

  const styles = useStyles((theme) => ({
    form: theme().css({
      input: {
        background: 'rgba(255,255,255,0.1)',
        padding: '5px 7px',
        borderRadius: '5px',
        margin: '5px 0 20px 0',
      },
      button: {
        background: 'rgna(0,0,0,0.1)',
        padding: '5px',
        borderRadius: '5px',
      },
    }),
  }))

  return (
    <section>
      <PageTitle title='EventPublisher' />
      <form css={styles.form} onSubmit={handlePublish}>
        <TextInput label='name' regist={register('name')} />
        {valueIds.map((vid, i) => {
          return (
            <div key={vid}>
              <TextInput label='name' regist={register(`kvs.${i}.name`)} />
              <TextInput label='value' regist={register(`kvs.${i}.value`)} />
              <button
                onClick={(e) => {
                  e.preventDefault()
                  removeValue(vid)
                }}
              >
                <FaMinus />
              </button>
            </div>
          )
        })}
        <button onClick={handleAddValue}>
          <FaPlus />
        </button>
        <button type='submit'>publish</button>
      </form>
    </section>
  )
}
