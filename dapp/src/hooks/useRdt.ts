import { useContext } from "react";
import { RdtContext } from "../radix/rdt-context";

export const useRdt = () => useContext(RdtContext)!