'use client'
import { useState } from 'react'

export function PhotoUploader({ obituaryId, onUploaded }: { obituaryId: string, onUploaded: (urls: string[]) => void }) {
  const [uploading, setUploading] = useState(false)

  const handleFiles = async (e: React.ChangeEvent<HTMLInputElement>) => {
    const files = e.target.files
    if (!files) return
    setUploading(true)
    const form = new FormData()
    for (let i=0;i<files.length;i++) form.append('photos', files[i])

    const res = await fetch('http://localhost:8080/api/v1/uploads/photo', { method:'POST', body: form })
    const data = await res.json()
    if (data.uploaded) onUploaded(data.uploaded.map((u:any)=>u.url))
    setUploading(false)
  }

  return (
    <div className="border-2 border-dashed border-zinc-700 rounded-xl p-6 text-center">
      <input type="file" multiple accept="image/*" onChange={handleFiles} className="hidden" id="photo-upload" />
      <label htmlFor="photo-upload" className="cursor-pointer">
        <div className="text-2xl">📸</div>
        <div className="mt-2 text-sm font-medium">{uploading ? 'Uploading to S3...' : 'Upload Photos - Real S3'}</div>
        <div className="text-xs text-zinc-500">S3 bucket: set S3_BUCKET, AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY. Works with AWS, R2, MinIO.</div>
      </label>
    </div>
  )
}
