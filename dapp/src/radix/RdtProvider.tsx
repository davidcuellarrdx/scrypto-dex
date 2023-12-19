import React, { ReactNode } from 'react'
import { RdtContext, Radix } from './rdt-context'

export const RdtProvider = ({
  value,
  children,
}: {
  value: Radix
  children: ReactNode
}) => <RdtContext.Provider value={value}>{children}</RdtContext.Provider>