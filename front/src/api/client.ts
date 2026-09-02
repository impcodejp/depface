const BASE = '/api'

export type LoginResponse = {
  token: string
  user_id: string
  user_name: string
}

export type RegisterResponse = {
  user_id: string
  user_name: string
  email: string
}

// ログイン認証API
export async function login(user_id: string, password: string): Promise<LoginResponse> {
  const res = await fetch(`${BASE}/auth/login`, {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ user_id, password }),
  })
  if (!res.ok) {
    const err = await res.json()
    throw new Error(err.error ?? 'ログインに失敗しました')
  }
  return res.json()
}

// ログアウトAPI
export async function logout(token: string): Promise<void> {
  const res = await fetch(`${BASE}/auth/logout`, {
    method: 'POST',
    headers: { Authorization: `Bearer ${token}` },
  })
  if (!res.ok) {
    const err = await res.json()
    throw new Error(err.error ?? 'ログアウトに失敗しました')
  }
}

// ユーザー登録API
export async function registerUser(
  token: string,
  data: { user_id: string; user_name: string; email: string; password: string }
): Promise<RegisterResponse> {
  const res = await fetch(`${BASE}/users`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(data),
  })
  if (!res.ok) {
    const err = await res.json()
    throw new Error(err.error ?? 'ユーザー追加に失敗しました')
  }
  return res.json()
}
