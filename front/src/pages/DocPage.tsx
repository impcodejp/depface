import ReactMarkdown from 'react-markdown'
import remarkGfm from 'remark-gfm'
import specRaw from '../../../SPEC.md?raw'

export default function DocPage() {
  return (
    <div className="min-h-screen bg-gray-50 py-10 px-6">
      <div className="max-w-4xl mx-auto bg-white rounded-2xl shadow-sm border border-gray-200 p-10">
        <article className="prose prose-slate max-w-none">
          <ReactMarkdown remarkPlugins={[remarkGfm]}>
            {specRaw}
          </ReactMarkdown>
        </article>
      </div>
    </div>
  )
}
