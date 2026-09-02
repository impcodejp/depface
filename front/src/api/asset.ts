// 物件関連のAPIクライアント

const BASE = '/api'

export type addAssetResponse = {
  id: number
  asset_id: string
  long_name: string
  short_name: string
}

// 物件登録API
export async function addAsset(
  token: string,
  data: {
    asset_code: string; long_name: string; short_name: string 
  }
): Promise<addAssetResponse> {
  const res = await fetch(`${BASE}/addasset`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      Authorization: `Bearer ${token}`,
    },
    body: JSON.stringify(data),
  })
  if (!res.ok) {
    const err = await res.json()
    throw new Error(err.error ?? '物件登録に失敗しました')
  }
  return res.json()
}