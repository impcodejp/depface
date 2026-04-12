import { useForm } from 'react-hook-form'
import AppLayout from '../components/AppLayout'

type PropertyForm = {
  property_code: string
  formal_name: string
  short_name: string
}

export default function PropertyAddPage() {
  const {
    register,
    handleSubmit,
    reset,
    formState: { errors, isSubmitting },
  } = useForm<PropertyForm>()

  async function onSubmit(data: PropertyForm) {
    try {
      // TODO: API 実装後に差し替え
      await new Promise((resolve) => setTimeout(resolve, 500))
      console.log('登録データ:', data)
      reset()
    } catch (err) {
      console.error(err)
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
                {...register('property_code', { required: '物件コードは必須です' })}
                className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent placeholder-gray-400"
              />
              {errors.property_code && (
                <p className="mt-1 text-xs text-red-600">{errors.property_code.message}</p>
              )}
            </div>

            <div>
              <label className="block text-sm font-medium text-gray-700 mb-1.5">
                正式名称
              </label>
              <input
                type="text"
                placeholder="○○マンション △△号室"
                {...register('formal_name', { required: '正式名称は必須です' })}
                className="w-full border border-gray-300 rounded-lg px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent placeholder-gray-400"
              />
              {errors.formal_name && (
                <p className="mt-1 text-xs text-red-600">{errors.formal_name.message}</p>
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
