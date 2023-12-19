import { useEffect, useState } from 'react'
import { useRdt } from './useRdt'

export const useConnected = () => {
  const rdt = useRdt()
  const [state, setState] = useState<
    'pending' | 'success' | 'error' | 'default'
  >('default')

  useEffect(() => {
    const subscription = rdt.buttonApi.status$.subscribe((state) => {
      setState(state)
    })

    return () => {
      subscription.unsubscribe()
    }
  }, [rdt])

  return state
}