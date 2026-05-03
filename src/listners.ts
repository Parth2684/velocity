import { listen } from '@tauri-apps/api/event';
import { store } from "./stores/useStore"
import { AvailableDevice, GetMetadata } from './stores/types';


export const setupListeners = async () => {
  const unlisteners: (() => void)[] = [];
  const { addAvailableDevice, removeAvailableDevice, setOtp, addSendTransfer, addReceiveTransfer } = store()
  const unlistenAddAvailableDevice = await listen<AvailableDevice>("add_available_device", (data) => {
    addAvailableDevice(data.payload)
  });
  unlisteners.push(unlistenAddAvailableDevice)
  
  const unlistenRemoveAvailableDevice = await listen<string>("remove_available_device", (name) => {
    removeAvailableDevice(name.payload);
  })
  unlisteners.push(unlistenRemoveAvailableDevice)
  
  const unlistenOtp = await listen<string>("connect_otp", (otp) => {
    setOtp(otp.payload)
  })
  unlisteners.push(unlistenOtp)
  
  const unlistenSend = await listen<Map<string, GetMetadata>>("to_send", (map) => {
    addSendTransfer(map.payload)
  })
  unlisteners.push(unlistenSend)
  
  const unlistenReceive = await listen<Map<string, GetMetadata>>("to_receive", (map) => {
    addReceiveTransfer(map.payload)
  })
  unlisteners.push(unlistenReceive)
  
  return () => {
    unlisteners.forEach((fn) => fn())
  }
}