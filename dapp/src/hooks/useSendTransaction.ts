import { useCallback } from 'react'
import { useRdt } from './useRdt'

export const useSendTransaction = () => {
  const rdt = useRdt()

  const sendTransaction = (transactionManifest: string, message?: string) =>
    rdt.walletApi.sendTransaction({
      transactionManifest,
      version: 1,
      message,
    })

  return useCallback(sendTransaction, [rdt])
}