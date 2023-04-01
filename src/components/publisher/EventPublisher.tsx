import { css, useTheme } from '@emotion/react'
import { PageTitle } from '@/components/common/PageTitle'
import { useEventPublishLazy } from '@/commands/event'
import { useState, FormEventHandler, MouseEventHandler } from 'react'
import { FaPlus, FaMinus } from 'react-icons/fa'
import { nanoid } from 'nanoid'
import { useForm } from 'react-hook-form'
import { TextInput } from '@/components/common/TextInput'

type FormDataKv = {
  name: string;
  value: string;
}
type FormData = {
  name: string;
  value: FormDataKv[];
}
export const EventPublisher = () => {
  const theme = useTheme()
  const [valueIds, setValueIds] = useState<string[]>([])

  const styles = {
    form: css({
      'input': { 
        ...theme.input,
        background: 'rgba(255,255,255,0.1)',
        padding: '5px 7px',
        borderRadius: '5px',
        color: theme.color.main,
        margin: '5px 0 20px 0',
        fontSize: theme.fontSize.large,
      },
      'button': {
        ...theme.input,
        background: 'rgna(0,0,0,0.1)',
        padding: '5px',
        borderRadius: '5px',
      },
    }),
  }

  const { invoke } = useEventPublishLazy()
  const handleAddValue: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault()
    setValueIds([...valueIds, nanoid()])
  }
  const removeValue = (id: string) => {
    setValueIds(valueIds.filter(v => v !== id));
  }

  const { register, handleSubmit } = useForm<FormData>()
  const handlePublish = handleSubmit((data) => {
    invoke({ req: data })
  })

  return (
    <section>
      <PageTitle title='EventPublisher' />
      <form css={styles.form} onSubmit={handlePublish}>
        <label htmlFor='eventName'>eventName</label>
        <TextInput label='name' regist={register('name')} />
        {valueIds.map((vid, i) => {
          return (
            <div key={vid}>
              <TextInput label='name' regist={register(`value.${i}.name`)} />
              <TextInput label='value' regist={register(`value.${i}.value`)} />
              <button onClick={e => { e.preventDefault(); removeValue(vid) }}><FaMinus /></button>
            </div>
          )
        })}
        <button onClick={handleAddValue}><FaPlus /></button>
        <button type='submit'>publish</button>
      </form>
    </section>
  )
}