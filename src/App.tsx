import { BrowserRouter, Routes, Route } from 'react-router-dom'
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
        <BrowserRouter>
          <Routes>
            <Route path='/' element={<TopPage />} />
            <Route path='/logs' element={<LogsPage />} />
            <Route path='/logs/:name' element={<LogViewPage />} />
          </Routes>
        </BrowserRouter>
      </Theme>
    </QueryClientProvider>
  )
}
