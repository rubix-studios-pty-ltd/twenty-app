import Image from 'next/image'

import { Instance } from '@/components/instance'

import twenty from '@/src-tauri/icons/icon.png'

export default function Page() {
  return (
    <main className="min-h-screen">
      <div className="mx-auto flex flex-col items-center justify-center min-h-screen w-full gap-6 p-4">
        <div className="flex flex-col items-center justify-center gap-6 border border-border bg-foreground text-background p-16 rounded-xl">
          <Image src={twenty} alt="Twenty" className="size-20 rounded-xl shadow-lg" loading="eager" />
          <Instance />
        </div>
      </div>
    </main>
  )
}
