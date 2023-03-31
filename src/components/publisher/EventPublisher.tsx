import { css, useTheme } from '@emotion/react'
import { PageTitle } from '@/components/common/PageTitle'
import { useEventPublishLazy } from '@/commands/event'
import { useState, FormEventHandler, MouseEventHandler } from 'react'
import { FaPlus, FaMinus } from 'react-icons/fa'
import { nanoid } from 'nanoid'

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

  const handlePublish: FormEventHandler<HTMLFormElement> = (e) => {
    e.preventDefault()
    const formElms = e.currentTarget.elements
    const name = (formElms.namedItem('eventName') as HTMLInputElement).value ?? ''
    const value: {name: string, value: string}[] = [];
    valueIds.map(vid => {
      const k = (formElms.namedItem(`value${vid}key`) as HTMLInputElement).value ?? ''
      const v = (formElms.namedItem(`value${vid}value`) as HTMLInputElement).value ?? ''
      value.push({ name: k, value: v });
    })
    invoke({ req: { name, value }})
  }

  return (
    <section>
      <PageTitle title='EventPublisher' />
      <form css={styles.form} onSubmit={handlePublish}>
        <label htmlFor='eventName'>eventName</label>
        <input type='text' name='eventName' />
        <div>
          {valueIds.map(vid => {
            return (
              <div key={vid}>
                <label htmlFor={`value${vid}key`}>key</label>
                <input type='text' name={`value${vid}key`}></input>
                <label htmlFor={`value${vid}value`}>value</label>
                <input type='text' name={`value${vid}value`} />
                <button onClick={e => { e.preventDefault(); removeValue(vid) }}><FaMinus /></button>
              </div>
            )
          })}
        </div>

        <button onClick={handleAddValue}><FaPlus /></button>
        <button type='submit'>publish</button>
      </form>
    </section>
  )
}