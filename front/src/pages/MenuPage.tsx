import { useNavigate } from 'react-router-dom'
import { useAuth } from '../contexts/AuthContext'
import { categories } from '../components/AppLayout'
import AppLayout from '../components/AppLayout'

function greeting() {
  const h = new Date().getHours()
  if (h < 12) return 'おはようございます'
  if (h < 18) return 'こんにちは'
  return 'お疲れ様です'
}

export default function MenuPage() {
  const { auth } = useAuth()
  const navigate = useNavigate()

  return (
    <AppLayout>
      <div className="p-8 max-w-3xl">
        {/* Greeting */}
        <div className="mb-10">
          <p className="text-gray-400 text-sm">{greeting()}</p>
          <h1 className="text-2xl font-bold text-gray-800 mt-0.5">
            {auth?.user_id} さん
          </h1>
        </div>

        {/* Category cards */}
        <div>
          <p className="text-xs font-semibold text-gray-400 uppercase tracking-widest mb-4">
            カテゴリ
          </p>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            {categories.map((cat) => {
              const Icon = cat.icon
              return (
                <div
                  key={cat.id}
                  className="bg-white rounded-xl border border-gray-200 p-5 hover:border-blue-300 hover:shadow-sm transition-all cursor-default"
                >
                  <div className="flex items-center gap-3 mb-4">
                    <div className="w-8 h-8 rounded-lg bg-blue-50 flex items-center justify-center">
                      <Icon size={16} className="text-blue-600" />
                    </div>
                    <span className="font-semibold text-gray-800 text-sm">{cat.label}</span>
                  </div>
                  <div className="flex flex-col gap-1">
                    {cat.items.map((item) => {
                      const ItemIcon = item.icon
                      return (
                        <button
                          key={item.path}
                          onClick={() => navigate(item.path)}
                          className="flex items-center gap-2 text-sm text-gray-500 hover:text-blue-600 py-1 px-2 rounded-lg hover:bg-blue-50 transition-colors text-left"
                        >
                          <ItemIcon size={13} />
                          {item.label}
                        </button>
                      )
                    })}
                  </div>
                </div>
              )
            })}
          </div>
        </div>
      </div>
    </AppLayout>
  )
}
