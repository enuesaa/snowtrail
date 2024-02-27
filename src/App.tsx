import { Route, Switch } from 'wouter'
import TopPage from '@/pages/index'
import LogsPage from '@/pages/logs/index'
import LogViewPage from '@/pages/logs/view'
import { QueryClient, QueryClientProvider } from 'react-query'
import '@radix-ui/themes/styles.css'
import { Theme } from '@radix-ui/themes'
import '@/styles/app.css'

export const App = () => {
  const queryClient = new QueryClient()

  return (
    <QueryClientProvider client={queryClient}>
      <Theme appearance='dark' accentColor='cyan'>
        <Switch>
          <Route path='/' component={TopPage} />
          <Route path='/logs' component={LogsPage} />
          <Route path='/logs/:name' component={LogViewPage} />
        </Switch>
      </Theme>
    </QueryClientProvider>
  )
}
