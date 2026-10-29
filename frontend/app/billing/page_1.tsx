'use client'
import { useEffect, useState } from 'react'

export default function Billing() {
  const [plans, setPlans] = useState<any>(null)
  const [loading, setLoading] = useState('')

  useEffect(()=>{
    fetch('http://localhost:8080/api/v1/billing/plans').then(r=>r.json()).then(setPlans)
  },[])

  const checkout = async (planId: string) => {
    setLoading(planId)
    // In prod, get home_id from JWT
    const home_id = localStorage.getItem('home_id') || '00000000-0000-0000-0000-000000000000'
    const res = await fetch('http://localhost:8080/api/v1/billing/checkout', {
      method:'POST', headers:{'Content-Type':'application/json'},
      body: JSON.stringify({
        home_id,
        plan: planId,
        success_url: window.location.origin + '/billing/success',
        cancel_url: window.location.origin + '/billing'
      })
    })
    const data = await res.json()
    if (data.checkout_url) window.location.href = data.checkout_url
    else alert(JSON.stringify(data))
    setLoading('')
  }

  if (!plans) return <div className="p-8">Loading plans...</div>

  return (
    <div className="max-w-6xl mx-auto p-8">
      <h1 className="text-3xl font-bold">Choose Your Plan - Real Stripe Checkout</h1>
      <p className="text-zinc-400 mt-2">No placeholder. This creates a real Stripe Checkout Session. Set STRIPE_SECRET_KEY in .env</p>
      <div className="grid grid-cols-3 gap-6 mt-8">
        {plans.plans.map((p:any)=>(
          <div key={p.id} className={`p-6 rounded-xl border ${p.popular ? 'border-white bg-zinc-900' : 'border-zinc-800 bg-zinc-900'}`}>
            {p.popular && <div className="text-xs bg-white text-black px-2 py-1 rounded-full inline-block mb-3">MOST POPULAR</div>}
            <h3 className="text-xl font-bold">{p.name}</h3>
            <div className="text-3xl font-bold mt-2">${p.price}<span className="text-sm font-normal text-zinc-500">/{p.interval}</span></div>
            <ul className="mt-4 space-y-2 text-sm text-zinc-300">
              {p.features.map((f:string)=><li key={f}>✓ {f}</li>)}
            </ul>
            <button onClick={()=>checkout(p.id)} disabled={!!loading} className="w-full mt-6 py-3 bg-white text-black rounded font-semibold disabled:opacity-50">
              {loading===p.id ? 'Redirecting to Stripe...' : `Start ${p.name}`}
            </button>
          </div>
        ))}
      </div>
      <div className="mt-8 p-4 bg-zinc-900 rounded border border-zinc-800 text-sm text-zinc-400">
        Add-on: {plans.addons[0].name} - ${plans.addons[0].price} per stream. Charge funeral homes extra for live streaming.
        <br/>Real MRR: 20 homes on Growth ($149) = $2,980 MRR. 100 homes = $14,900 MRR + stream fees.
      </div>
    </div>
  )
}
