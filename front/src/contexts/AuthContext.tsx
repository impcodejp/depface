import { createContext, useContext, useState } from 'react'
import type { ReactNode } from 'react'

type AuthState = {
  token: string
  user_id: string
} | null

type AuthContextType = {
  auth: AuthState
  setAuth: (auth: AuthState) => void
}

const AuthContext = createContext<AuthContextType | null>(null)

export function AuthProvider({ children }: { children: ReactNode }) {
  const [auth, setAuth] = useState<AuthState>(null)
  return <AuthContext.Provider value={{ auth, setAuth }}>{children}</AuthContext.Provider>
}

export function useAuth() {
  const ctx = useContext(AuthContext)
  if (!ctx) throw new Error('useAuth must be used within AuthProvider')
  return ctx
}
