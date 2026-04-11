import { useState, useEffect } from 'react'
import { NavLink, useNavigate, useLocation } from 'react-router-dom'
import { useAuth } from '../contexts/AuthContext'
import { logout } from '../api/client'
import { Users, UserPlus, LogOut, Home } from 'lucide-react'

export type NavItem = {
  label: string
  path: string
  icon: React.ElementType
}

export type Category = {
  id: string
  label: string
  shortLabel: string
  icon: React.ElementType
  basePath: string
  items: NavItem[]
}

export const categories: Category[] = [
  {
    id: 'users',
    label: 'ユーザー管理',
    shortLabel: 'ユーザー',
    icon: Users,
    basePath: '/users',
    items: [
      { label: 'ユーザー追加', path: '/users/add', icon: UserPlus },
    ],
  },
]

export default function AppLayout({ children }: { children: React.ReactNode }) {
  const { auth, setAuth } = useAuth()
  const navigate = useNavigate()
  const location = useLocation()

  const activeCategory = categories.find((c) =>
    location.pathname.startsWith(c.basePath)
  ) ?? null

  const [openCategory, setOpenCategory] = useState<Category | null>(activeCategory)

  useEffect(() => {
    if (activeCategory) setOpenCategory(activeCategory)
  }, [location.pathname])

  function handleCategoryClick(cat: Category) {
    setOpenCategory((prev) => (prev?.id === cat.id ? null : cat))
  }

  async function handleLogout() {
    if (!auth) return
    try {
      await logout(auth.token)
    } finally {
      setAuth(null)
      navigate('/')
    }
  }

  return (
    <div className="flex h-screen bg-gray-100">
      {/* カテゴリアイコンサイドバー */}
      <aside className="w-16 bg-slate-900 flex flex-col items-stretch py-4 gap-0.5 z-10 flex-shrink-0">
        {/* Logo */}
        <div className="flex justify-center mb-3">
          <div className="w-9 h-9 rounded-xl bg-blue-600 flex items-center justify-center">
            <span className="text-white text-xs font-bold tracking-tight">df</span>
          </div>
        </div>

        {/* Home */}
        <div className="relative flex justify-center px-2 mb-3">
          <NavLink
            to="/menu"
            title="メインメニュー"
            className={({ isActive }) =>
              `w-full flex flex-col items-center justify-center gap-0.5 py-2 rounded-lg transition-colors ${
                isActive
                  ? 'bg-slate-700 text-white'
                  : 'text-slate-500 hover:bg-slate-800 hover:text-slate-300'
              }`
            }
          >
            {({ isActive }) => (
              <>
                <span
                  className={`absolute left-0 top-1 bottom-1 w-0.5 rounded-r-full bg-blue-500 transition-opacity duration-150 ${
                    isActive ? 'opacity-100' : 'opacity-0'
                  }`}
                />
                <Home size={17} />
                <span className="text-[9px] font-medium leading-none">ホーム</span>
              </>
            )}
          </NavLink>
        </div>

        {/* Divider */}
        <div className="mx-3 mb-2 border-t border-slate-700" />

        {/* Category buttons */}
        {categories.map((cat) => {
          const Icon = cat.icon
          const isOpen = openCategory?.id === cat.id
          return (
            <div key={cat.id} className="relative flex justify-center px-2">
              {/* Left accent bar */}
              <span
                className={`absolute left-0 top-1 bottom-1 w-0.5 rounded-r-full bg-blue-500 transition-opacity duration-150 ${
                  isOpen ? 'opacity-100' : 'opacity-0'
                }`}
              />
              <button
                onClick={() => handleCategoryClick(cat)}
                title={cat.label}
                className={`w-full flex flex-col items-center justify-center gap-0.5 py-2 rounded-lg transition-colors ${
                  isOpen
                    ? 'bg-slate-700 text-white'
                    : 'text-slate-500 hover:bg-slate-800 hover:text-slate-300'
                }`}
              >
                <Icon size={17} />
                <span className="text-[9px] font-medium leading-none">{cat.shortLabel}</span>
              </button>
            </div>
          )
        })}

        <div className="flex-1" />

        {/* User + logout */}
        <div className="flex flex-col items-center gap-3 pb-1">
          <div
            className="w-8 h-8 rounded-full bg-slate-700 ring-2 ring-slate-600 flex items-center justify-center cursor-default"
            title={auth?.user_id}
          >
            <span className="text-slate-200 text-xs font-bold">
              {auth?.user_id?.charAt(0).toUpperCase()}
            </span>
          </div>
          <button
            onClick={handleLogout}
            title="ログアウト"
            className="text-slate-500 hover:text-slate-200 transition-colors"
          >
            <LogOut size={15} />
          </button>
        </div>
      </aside>

      {/* サブメニューパネル */}
      <div
        className={`bg-slate-800 flex flex-col flex-shrink-0 transition-all duration-200 overflow-hidden ${
          openCategory ? 'w-52' : 'w-0'
        }`}
      >
        {openCategory && (
          <>
            {/* Category header */}
            <div className="px-4 pt-6 pb-4 border-b border-slate-700 flex items-center gap-3">
              <div className="w-7 h-7 rounded-lg bg-blue-600 flex items-center justify-center flex-shrink-0">
                <openCategory.icon size={14} className="text-white" />
              </div>
              <p className="text-slate-200 text-sm font-semibold">{openCategory.label}</p>
            </div>

            {/* Nav items */}
            <nav className="flex-1 px-2 py-3 flex flex-col gap-0.5">
              {openCategory.items.map(({ label, path, icon: Icon }) => (
                <NavLink
                  key={path}
                  to={path}
                  className={({ isActive }) =>
                    `flex items-center gap-2.5 px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                      isActive
                        ? 'bg-blue-600 text-white'
                        : 'text-slate-400 hover:bg-slate-700 hover:text-slate-100'
                    }`
                  }
                >
                  <Icon size={14} />
                  {label}
                </NavLink>
              ))}
            </nav>
          </>
        )}
      </div>

      {/* メインコンテンツ */}
      <main className="flex-1 overflow-y-auto">
        {children}
      </main>
    </div>
  )
}
