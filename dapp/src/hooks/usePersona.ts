import { useEffect, useState } from 'react'
import { useRdt } from './useRdt'
import { Persona } from '@radixdlt/radix-dapp-toolkit'

export const usePersona = () => {
  const rdt = useRdt()
  const [state, setState] = useState<{
    persona?: Persona
    hasLoaded: boolean
  }>({ hasLoaded: false })

  useEffect(() => {
    const subscription = rdt.walletApi.walletData$.subscribe(
      (state) => {
        setState({ persona: state.persona, hasLoaded: true })
      }
    )

    return () => {
      subscription.unsubscribe()
    }
  }, [rdt])

  return state
}