import { listen } from '@tauri-apps/api/event';
import { store } from "./stores/useStore"
import { AvailableDevice, GetMetadata, GetProgress, TransferType } from './stores/types';



export const setupListeners = async () => {
  const unlisteners: (() => void)[] = [];
  const { addAvailableDevice, removeAvailableDevice, setOtp, addSendTransfer, addReceiveTransfer, updateProgress, cancelReceive } = store()
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
  
  const unlistenSendProgress = await listen<GetProgress>("send_progress", (progress) => {
    updateProgress(progress.payload, TransferType.Send)
  })
  unlisteners.push(unlistenSendProgress)
  
  const unlistenReceiveProgress = await listen<GetProgress>("receive_progress", (progress) => {
    updateProgress(progress.payload, TransferType.Receive)
  })
  unlisteners.push(unlistenReceiveProgress)
  
  const unlistenCancelReceive = await listen<string>("receive_stop", (path) => {
    cancelReceive(path.payload)
  })
  unlisteners.push(unlistenCancelReceive)
  
  return () => {
    unlisteners.forEach((fn) => fn())
  }
}