'use client'

import { useCallback, useEffect, useState } from 'react'
import { toast } from 'sonner'

import { Button } from '@/components/ui/button'
import { Input } from '@/components/ui/input'
import { Label } from '@/components/ui/label'
import { Spinner } from '@/components/ui/spinner'
import { readUrl } from '@/lib/tauri/settings'
import { useStore } from '@/store/instance'
import { startInstance } from '@/utils/startInstance'

export function Instance() {
  const [loading, setLoading] = useState(true)

  const instance = useStore((state) => state.instance)
  const setInstance = useStore((state) => state.setInstance)

  const handleConnect = useCallback((url: string | null) => {
    if (!url) return
    startInstance(url)
  }, [])

  useEffect(() => {
    readUrl()
      .then((savedUrl) => {
        setInstance(savedUrl)

        if (savedUrl) {
          handleConnect(savedUrl)
          return
        }

        setLoading(false)
      })
      .catch(() => {
        setLoading(false)
      })
  }, [handleConnect, setInstance])

  if (loading) {
    return (
      <div className="min-w-xs min-h-33 gap-6 flex flex-col items-center">
        <Label className="font-bold text-lg justify-center w-full">Twenty</Label>
        <Spinner className="size-6" />
      </div>
    )
  }

  return (
    <form
      className="grid gap-4 justify-center min-w-xs"
      onSubmit={(event) => {
        event.preventDefault()
        toast.success('Connecting')
        handleConnect(instance)
      }}
    >
      <Label className="font-bold text-lg justify-center w-full">Twenty</Label>
      <Input
        className="border border-border text-xs"
        name="instanceUrl"
        type="url"
        inputMode="url"
        autoComplete="off"
        spellCheck={false}
        value={instance || ''}
        placeholder="https://app.twenty.com"
        required
        onChange={(event) => setInstance(event.target.value.trim())}
      />

      <Button
        className="flex-1 cursor-pointer transition-all duration-300 text-foreground bg-black hover:bg-black/60"
        type="submit"
      >
        Start
      </Button>
    </form>
  )
}
