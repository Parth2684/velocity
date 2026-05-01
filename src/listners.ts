import { listen } from '@tauri-apps/api/event';
import { store } from "./stores/useStore"
import { AvailableDevice } from './stores/types';


export const setupListeners = async () => {
  const unlisteners: (() => void)[] = [];
  const { addAvailableDevice, removeAvailableDevice } = store()
  const unlistenAddAvailableDevice = await listen<AvailableDevice>("add_available_device", (data) => {
    addAvailableDevice(data.payload)
  });
  unlisteners.push(unlistenAddAvailableDevice)
  
  const unlistenRemoveAvailableDevice = await listen<string>("remove_available_device", (name) => {
    removeAvailableDevice(name.payload);
  })
  unlisteners.push(unlistenRemoveAvailableDevice)
  
  
}