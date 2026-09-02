import { useForm } from 'react-hook-form'
import AppLayout from '../components/AppLayout'
import { addAsset } from '../api/asset'
import { useAuth } from '../contexts/AuthContext'


type PropertyForm = {
  asset_code: string
  long_name: string
  short_name: string
}

export default function AssetAddPage() {
  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isSubmitting },
  } = useForm<PropertyForm>()

  const { auth } = useAuth()

  async function onSubmit(data: PropertyForm) {
    try {
      if (!auth) throw new Error('認証情報がありません')

      // ここでAPI呼び出しを行う
      const res = await addAsset(auth.token, data)
      alert(`物件「${res.long_name}」を登録しました`)
      reset()
    } catch (error) {
      console.error(error)
      alert('物件の登録に失敗しました')
    }
  }

  return (
    <AppLayout>
      <div className="p-8">
        <div className="mb-8">
          <h1 className="text-2xl font-bold text-gray-900">物件登録</h1>
          <p className="text-gray-500 mt-1 text-sm">新しい物件をシステムに登録します</p>
        </div>

        <div className="bg-white rounded-xl border border-gray-200 p-6 max-w-lg">
          <form onSubmit={handleSubmit(onSubmit)} className="flex flex-col gap-5">

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1.5">
                物件コード
              </label>
              <input
                type="text"
                placeholder="P-001"
                {...register('asset_code', { required: '物件コードは必須です' })}
                className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent placeholder-gray-400"
              />
              {errors.asset_code && (
                <p className="mt-1 text-xs text-red-600">{errors.asset_code.message}</p>
              )}
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1.5">
                正式名称
              </label>
              <input
                type="text"
                placeholder="○○マンション △△号室"
                {...register('long_name', { required: '正式名称は必須です' })}
                className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent placeholder-gray-400"
              />
              {errors.long_name && (
                <p className="mt-1 text-xs text-red-600">{errors.long_name.message}</p>
              )}
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1.5">
                簡略名称
              </label>
              <input
                type="text"
                placeholder="○○MN"
                {...register('short_name', { required: '簡略名称は必須です' })}
                className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent placeholder-gray-400"
              />
              {errors.short_name && (
                <p className="mt-1 text-xs text-red-600">{errors.short_name.message}</p>
              )}
            </div>

            <div className="pt-1">
              <button
                type="submit"
                disabled={isSubmitting}
                className="w-full bg-blue-600 text-white py-2.5 rounded-lg text-sm font-medium hover:bg-blue-700 disabled:opacity-50 transition-colors"
              >
                {isSubmitting ? '処理中...' : '物件を登録'}
              </button>
            </div>

          </form>
        </div>
      </div>
    </AppLayout>
  )
}
