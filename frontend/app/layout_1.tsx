import './globals.css'
export const metadata = { title: 'FuneralOS - Funeral Home Operating System', description: 'Boring but stable $1.5B -> $3B vertical' }
export default function RootLayout({ children }: { children: React.ReactNode }) {
  return <html lang="en"><body className="min-h-screen bg-zinc-950 text-zinc-100 antialiased">{children}</body></html>
}
