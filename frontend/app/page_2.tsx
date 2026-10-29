'use client'
import { useEffect, useState } from 'react'

export default function Dashboard() {
  const [stats, setStats] = useState<any>(null)
  const [obits, setObits] = useState<any[]>([])

  useEffect(() => {
    fetch('http://localhost:8080/api/v1/dashboard/stats').then(r=>r.json()).then(setStats).catch(()=>{})
    fetch('http://localhost:8080/api/v1/obituaries').then(r=>r.json()).then(setObits).catch(()=>{})
  }, [])

  return (
    <div className="p-8 max-w-7xl mx-auto">
      <header className="flex justify-between items-center mb-8">
        <h1 className="text-3xl font-bold">FuneralOS <span className="text-zinc-500 text-lg">v1.0 Production</span></h1>
        <div className="flex gap-2">
          <a href="/obituaries/new" className="px-4 py-2 bg-white text-black rounded font-medium">+ New Obituary</a>
        </div>
      </header>

      <div className="grid grid-cols-4 gap-4 mb-8">
        <div className="bg-zinc-900 p-6 rounded-xl border border-zinc-800"><div className="text-zinc-400 text-sm">Active Memorials</div><div className="text-3xl font-bold mt-2">{stats?.obituaries ?? 0}</div></div>
        <div className="bg-zinc-900 p-6 rounded-xl border border-zinc-800"><div className="text-zinc-400 text-sm">Live Streams</div><div className="text-3xl font-bold mt-2">{stats?.streams ?? 0}</div></div>
        <div className="bg-zinc-900 p-6 rounded-xl border border-zinc-800"><div className="text-zinc-400 text-sm">Guestbook</div><div className="text-3xl font-bold mt-2">{stats?.guestbook_entries ?? 0}</div><div className="text-xs text-amber-400 mt-1">{stats?.pending_moderation ?? 0} pending</div></div>
        <div className="bg-zinc-900 p-6 rounded-xl border border-zinc-800"><div className="text-zinc-400 text-sm">Est. MRR</div><div className="text-3xl font-bold mt-2">${stats?.mrr_estimate ?? 0}</div><div className="text-xs text-zinc-500">at $49/memorial</div></div>
      </div>

      <div className="bg-zinc-900 rounded-xl border border-zinc-800">
        <div className="p-6 border-b border-zinc-800 flex justify-between"><h2 className="font-semibold">Recent Obituaries</h2><a href="/obituaries" className="text-sm text-zinc-400">View all</a></div>
        <div className="divide-y divide-zinc-800">
          {obits.map((o:any)=>(
            <div key={o.id} className="p-6 flex justify-between items-center hover:bg-zinc-800/50">
              <div><div className="font-medium">{o.data?.full_name || JSON.parse(o.data||'{}').full_name || 'Unnamed'}</div><div className="text-sm text-zinc-500">{new Date(o.created_at).toLocaleDateString()} • {o.data?.status || 'draft'}</div></div>
              <a href={`/memorial/${o.id}`} className="text-sm px-3 py-1.5 bg-zinc-800 rounded">Manage</a>
            </div>
          ))}
          {obits.length===0 && <div className="p-12 text-center text-zinc-500">No obituaries yet. Create your first to start selling.</div>}
        </div>
      </div>

      <div className="mt-8 grid grid-cols-2 gap-4">
        <div className="bg-zinc-900 p-6 rounded-xl border border-zinc-800"><h3 className="font-semibold mb-2">🎥 Live Streaming</h3><p className="text-sm text-zinc-400">RTMPS ingest via Mux. Each memorial gets unique stream key + HLS playback. Funeral homes charge $199 per live stream add-on.</p></div>
        <div className="bg-zinc-900 p-6 rounded-xl border border-zinc-800"><h3 className="font-semibold mb-2">📜 Compliance</h3><p className="text-sm text-zinc-400">Utah + California EDRS ready. Encrypted SSN + cause. Tracking numbers auto-generated. Audit log immutable.</p></div>
      </div>
    </div>
  )
}
