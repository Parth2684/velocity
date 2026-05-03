import { useEffect, useState } from 'react';
import { store } from '../stores/useStore';
import { AvailableDevice, Discovery } from '../stores/types';
import { Button } from '../components/ui/button';
import { listen } from '@tauri-apps/api/event';
import { useNavigate } from 'react-router';

export default function Scan() {
  const { scan, availableDevices, receiveCertAndConnectQuic, addConnectedTo, connectedTo } = store()
  const [device, setDevice] = useState<AvailableDevice>()
  const [showInput, setShowInput] = useState(false);
  const [otp, setOtp] = useState<string>("");
  const listenToEvent = async () => {
    const unlisten = await listen<string>("connection_success", (device) => {
      addConnectedTo(device.payload);
    });
    return () => {
      unlisten();
    };
  }; 
  
  const nav = useNavigate();
  
  useEffect(() => {
    if (connectedTo !== null) {
      nav("/transfer")
    }
    let cleanup: () => void;
    listenToEvent().then((fn) => {
      cleanup = fn;
    });
    return () => {
      if (cleanup) cleanup();
    };
  }, [connectedTo]);
  
  useEffect(() => {
    const scanFn = async () => {
      await scan(Discovery.On)
    }
    scanFn()
  }, [])
  return <div>
    {Array.from(availableDevices.entries()).map(([id, device]) => (
      <div key={id}> <Button onClick={() => {
        setDevice(device);
        setShowInput(true)
      }}>{ JSON.stringify(device) }</Button> </div>
    ))}
    {showInput ?? <div>
      <input type='text' onChange={(e) => setOtp(e.target.value)}></input>
      <Button type='submit' onClick={async() => {
        receiveCertAndConnectQuic(device!.txt_properties, otp)
      }} />
    </div>}
  </div>
  
  
}