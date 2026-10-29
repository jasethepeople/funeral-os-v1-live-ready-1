'use client'
import { useEffect, useState } from 'react'
import MuxPlayer from '@mux/mux-player-react'

export default function MemorialPage({ params }: { params: { id: string } }) {
  const [obit, setObit] = useState<any>(null)
  const [guestbook, setGuestbook] = useState<any[]>([])
  const [stream, setStream] = useState<any>(null)
  const [newEntry, setNewEntry] = useState({ author_name: '', message: '' })

  useEffect(()=>{
    fetch(`http://localhost:8080/api/v1/obituaries/${params.id}`).then(r=>r.json()).then(setObit)
    fetch(`http://localhost:8080/api/v1/guestbook/${params.id}`).then(r=>r.json()).then(setGuestbook)
  },[params.id])

  const createStream = async ()=>{
    const res = await fetch('http://localhost:8080/api/v1/streams', { method:'POST', headers:{'Content-Type':'application/json'}, body: JSON.stringify({ obituary_id: params.id, title: `Service for ${obit?.data?.full_name || 'Memorial'}` }) })
    const data = await res.json(); setStream(data)
  }
  const publish = async ()=>{
    await fetch(`http://localhost:8080/api/v1/obituaries/${params.id}/publish`, {method:'POST'})
    alert('Published - public URL ready')
  }
  const signGuestbook = async ()=>{
    await fetch(`http://localhost:8080/api/v1/guestbook/${params.id}`, {method:'POST', headers:{'Content-Type':'application/json'}, body: JSON.stringify(newEntry)})
    setNewEntry({author_name:'', message:''})
    fetch(`http://localhost:8080/api/v1/guestbook/${params.id}`).then(r=>r.json()).then(setGuestbook)
  }

  if (!obit) return <div className="p-8">Loading memorial...</div>
  const data = obit.data

  return (
    <div className="max-w-6xl mx-auto p-8 grid grid-cols-3 gap-8">
      <div className="col-span-2 space-y-6">
        <div className="bg-zinc-900 p-8 rounded-xl border border-zinc-800">
          <h1 className="text-4xl font-bold">{data.full_name}</h1>
          <p className="text-zinc-400 mt-2">{data.date_of_birth} - {data.date_of_passing}</p>
          <p className="mt-6 leading-relaxed whitespace-pre-wrap">{data.biography?.substring(0,200) || 'Biography encrypted at rest - decrypted on authorized view'}</p>
          {data.service_date && <div className="mt-6 p-4 bg-zinc-950 rounded border border-zinc-800">Service: {new Date(data.service_date).toLocaleString()} at {data.service_location}</div>}
          <div className="mt-6 flex gap-2"><button onClick={publish} className="px-4 py-2 bg-white text-black rounded font-medium">Publish Public Page</button><button onClick={createStream} className="px-4 py-2 bg-zinc-800 rounded">Create Live Stream</button></div>
        </div>

        {stream && (
          <div className="bg-zinc-900 p-6 rounded-xl border border-amber-500/30">
            <h3 className="font-semibold mb-3">Live Stream Ready - Sell this for $199</h3>
            <div className="space-y-2 text-sm font-mono bg-zinc-950 p-4 rounded">
              <div>RTMP URL: {stream.rtmp_url}</div>
              <div>Stream Key: {stream.stream_key}</div>
              <div>Playback: {stream.playback_url}</div>
            </div>
            <div className="mt-4"><MuxPlayer playbackId={stream.playback_url.split('/').pop()?.replace('.m3u8','')} streamType="live" /></div>
            <p className="text-xs text-zinc-500 mt-2">Give RTMP to camera operator. Family watches via HLS. WebRTC room: {stream.webrtc_room}</p>
          </div>
        )}

        <div className="bg-zinc-900 p-6 rounded-xl border border-zinc-800">
          <h3 className="font-semibold mb-4">Virtual Guestbook ({guestbook.length})</h3>
          <div className="space-y-4 mb-6">
            {guestbook.map((g:any)=><div key={g.id} className="border-b border-zinc-800 pb-3"><div className="font-medium text-sm">{g.data.author_name} <span className="text-zinc-500">• {new Date(g.created_at).toLocaleDateString()}</span></div><div className="mt-1 text-sm">{g.data.message}</div><div className="text-xs text-zinc-500 mt-1">{g.status}</div></div>)}
          </div>
          <div className="bg-zinc-950 p-4 rounded border border-zinc-800 space-y-3">
            <input placeholder="Your Name" value={newEntry.author_name} onChange={e=>setNewEntry({...newEntry, author_name:e.target.value})} className="w-full p-2 bg-zinc-900 border border-zinc-800 rounded" />
            <textarea placeholder="Share a memory..." value={newEntry.message} onChange={e=>setNewEntry({...newEntry, message:e.target.value})} className="w-full p-2 bg-zinc-900 border border-zinc-800 rounded" rows={3} />
            <button onClick={signGuestbook} className="px-4 py-2 bg-white text-black rounded text-sm font-medium">Sign Guestbook</button>
          </div>
        </div>
      </div>

      <div className="space-y-6">
        <div className="bg-zinc-900 p-6 rounded-xl border border-zinc-800"><h3 className="font-semibold mb-3">Compliance</h3><a href={`/compliance/${params.id}`} className="block w-full text-center py-2 bg-zinc-800 rounded text-sm">File Death Certificate (EDRS)</a></div>
        <div className="bg-zinc-900 p-6 rounded-xl border border-zinc-800"><h3 className="font-semibold mb-3">Family Tree</h3><a href={`/family/${params.id}`} className="block w-full text-center py-2 bg-zinc-800 rounded text-sm">Manage Multi-Gen Records</a></div>
      </div>
    </div>
  )
}
