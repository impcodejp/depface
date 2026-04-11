import { useState } from 'react'
import type { FormEvent } from 'react'
import { registerUser } from '../api/client'
import { useAuth } from '../contexts/AuthContext'
import AppLayout from '../components/AppLayout'

const fields = [
  { label: 'ユーザーID', name: 'user_id', type: 'text', placeholder: 'user001' },
  { label: 'ユーザー名', name: 'user_name', type: 'text', placeholder: '山田 太郎' },
  { label: 'メールアドレス', name: 'email', type: 'email', placeholder: 'example@company.com' },
  { label: 'パスワード', name: 'password', type: 'password', placeholder: '8文字以上' },
] as const

type FormKey = (typeof fields)[number]['name']
type Form = Record<FormKey, string>

const emptyForm: Form = { user_id: '', user_name: '', email: '', password: '' }

export default function UserAddPage() {
  const { auth } = useAuth()
  const [form, setForm] = useState<Form>(emptyForm)
  const [error, setError] = useState('')
  const [success, setSuccess] = useState('')
  const [loading, setLoading] = useState(false)

  function handleChange(e: React.ChangeEvent<HTMLInputElement>) {
    setForm((prev) => ({ ...prev, [e.target.name]: e.target.value }))
  }

  async function handleSubmit(e: FormEvent) {
    e.preventDefault()
    if (!auth) return
    setError('')
    setSuccess('')
    setLoading(true)
    try {
      const res = await registerUser(auth.token, form)
      setSuccess(`ユーザー「${res.user_name}」を追加しました`)
      setForm(emptyForm)
    } catch (err) {
      setError(err instanceof Error ? err.message : 'エラーが発生しました')
    } finally {
      setLoading(false)
    }
  }

  return (
    <AppLayout>
      <div className="p-8">
        <div className="mb-8">
          <h1 className="text-2xl font-bold text-gray-900">ユーザー追加</h1>
          <p className="text-gray-500 mt-1 text-sm">新しいユーザーをシステムに登録します</p>
        </div>

        <div className="bg-white rounded-xl border border-gray-200 p-6 max-w-lg">
          <form onSubmit={handleSubmit} className="flex flex-col gap-5">
            {fields.map(({ label, name, type, placeholder }) => (
              <div key={name}>
                <label className="block text-sm font-medium text-gray-700 mb-1.5">
                  {label}
                </label>
                <input
                  type={type}
                  name={name}
                  value={form[name]}
                  onChange={handleChange}
                  placeholder={placeholder}
                  required
                  className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent placeholder-gray-400"
                />
              </div>
            ))}

            {error && (
              <div className="bg-red-50 border border-red-200 text-red-700 text-sm rounded-lg px-4 py-3">
                {error}
              </div>
            )}
            {success && (
              <div className="bg-green-50 border border-green-200 text-green-700 text-sm rounded-lg px-4 py-3">
                {success}
              </div>
            )}

            <div className="pt-1">
              <button
                type="submit"
                disabled={loading}
                className="w-full bg-blue-600 text-white py-2.5 rounded-lg text-sm font-medium hover:bg-blue-700 disabled:opacity-50 transition-colors"
              >
                {loading ? '処理中...' : 'ユーザーを追加'}
              </button>
            </div>
          </form>
        </div>
      </div>
    </AppLayout>
  )
}
