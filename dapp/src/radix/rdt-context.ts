import { createContext } from 'react'
import { RadixDappToolkit } from '@radixdlt/radix-dapp-toolkit'

export type Radix = ReturnType<typeof RadixDappToolkit>

export const RdtContext = createContext<Radix | null>(null)