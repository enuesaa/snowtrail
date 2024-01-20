import { BrowserRouter, Routes, Route } from 'react-router-dom'
import { Layout } from './components/common/Layout'
import { globalStyle } from './styles/global'
import { baseTheme } from './styles/theme'
import { Global, ThemeProvider } from '@emotion/react'
import TopPage from './pages/index'

export const App = () => {
  return (
    <>
      <Global styles={globalStyle} />
      <ThemeProvider theme={baseTheme}>
        <Layout>
          <BrowserRouter>
            <Routes>
              <Route path='/' element={<TopPage />} />
            </Routes>
          </BrowserRouter>
        </Layout>
      </ThemeProvider>
    </>
  )
}
