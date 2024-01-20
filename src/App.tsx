import { BrowserRouter, Routes, Route } from 'react-router-dom'
import TopPage from './pages/index'
import { QueryClient, QueryClientProvider } from 'react-query'
import '@radix-ui/themes/styles.css'
import { Theme } from '@radix-ui/themes'
import './styles/app.css'

export const App = () => {
  const queryClient = new QueryClient()

  return (
    <QueryClientProvider client={queryClient}>
      <Theme appearance='dark'>
        <BrowserRouter>
          <Routes>
            <Route path='/' element={<TopPage />} />
          </Routes>
        </BrowserRouter>
      </Theme>
    </QueryClientProvider>
  )
}
