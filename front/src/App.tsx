import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import { AuthProvider, useAuth } from './contexts/AuthContext'
import DocPage from './pages/DocPage'
import InterfacePage from './pages/InterfacePage'
import LoginPage from './pages/LoginPage'
import MenuPage from './pages/MenuPage'
import AssetAddPage from './pages/AssetAddPage'
import UserAddPage from './pages/UserAddPage'

function ProtectedRoute({ children }: { children: React.ReactNode }) {
  const { auth } = useAuth()
  return auth ? <>{children}</> : <Navigate to="/" replace />
}

function AppRoutes() {
  return (
    <Routes>
      <Route path="/" element={<LoginPage />} />
      <Route path="/doc/1" element={<DocPage />} />
      <Route path="/doc/2" element={<InterfacePage />} />
      <Route path="/menu" element={<ProtectedRoute><MenuPage /></ProtectedRoute>} />
      <Route path="/users/add" element={<ProtectedRoute><UserAddPage /></ProtectedRoute>} />
      <Route path="/assets/add" element={<ProtectedRoute><AssetAddPage /></ProtectedRoute>} />
    </Routes>
  )
}

export default function App() {
  return (
    <BrowserRouter>
      <AuthProvider>
        <AppRoutes />
      </AuthProvider>
    </BrowserRouter>
  )
}
