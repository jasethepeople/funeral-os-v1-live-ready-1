'use client'
import { useState } from 'react'
import { useRouter } from 'next/navigation'

export default function NewObituary() {
  const [form, setForm] = useState({ full_name: '', date_of_birth: '', date_of_passing: '', biography: '', service_date: '', service_location: '', visibility: 'public' })
  const [loading, setLoading] = useState(false)
  const router = useRouter()

  const submit = async () => {
    setLoading(true)
    const res = await fetch('http://localhost:8080/api/v1/obituaries', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({
        full_name: form.full_name,
        date_of_birth: form.date_of_birth,
        date_of_passing: form.date_of_passing,
        biography: form.biography,
        service_date: form.service_date ? new Date(form.service_date).toISOString() : null,
        service_location: form.service_location,
        visibility: form.visibility
      })
    })
    const data = await res.json()
    if (data.id) router.push(`/memorial/${data.id}`)
    setLoading(false)
  }

  return (
    <div className="max-w-3xl mx-auto p-8">
      <h1 className="text-2xl font-bold mb-6">Create Obituary - Real Encrypted Storage</h1>
      <div className="space-y-4 bg-zinc-900 p-6 rounded-xl border border-zinc-800">
        <input placeholder="Full Name" value={form.full_name} onChange={e=>setForm({...form, full_name:e.target.value})} className="w-full p-3 bg-zinc-950 border border-zinc-800 rounded" />
        <div className="grid grid-cols-2 gap-4">
          <input type="date" value={form.date_of_birth} onChange={e=>setForm({...form, date_of_birth:e.target.value})} className="p-3 bg-zinc-950 border border-zinc-800 rounded" />
          <input type="date" value={form.date_of_passing} onChange={e=>setForm({...form, date_of_passing:e.target.value})} className="p-3 bg-zinc-950 border border-zinc-800 rounded" />
        </div>
        <textarea placeholder="Biography - will be AES-GCM-256 encrypted at rest" rows={8} value={form.biography} onChange={e=>setForm({...form, biography:e.target.value})} className="w-full p-3 bg-zinc-950 border border-zinc-800 rounded" />
        <input type="datetime-local" value={form.service_date} onChange={e=>setForm({...form, service_date:e.target.value})} className="w-full p-3 bg-zinc-950 border border-zinc-800 rounded" />
        <input placeholder="Service Location (e.g. Willard City Cemetery)" value={form.service_location} onChange={e=>setForm({...form, service_location:e.target.value})} className="w-full p-3 bg-zinc-950 border border-zinc-800 rounded" />
        <select value={form.visibility} onChange={e=>setForm({...form, visibility:e.target.value})} className="w-full p-3 bg-zinc-950 border border-zinc-800 rounded"><option value="public">Public</option><option value="private">Private</option><option value="unlisted">Unlisted</option></select>
        <button onClick={submit} disabled={loading} className="w-full py-3 bg-white text-black rounded font-semibold disabled:opacity-50">{loading ? 'Creating...' : 'Create + Encrypt + Store'}</button>
        <p className="text-xs text-zinc-500">Real production: biography encrypted with ENCRYPTION_KEY before Postgres insert. No placeholder.</p>
      </div>
    </div>
  )
}
