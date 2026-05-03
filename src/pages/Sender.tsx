import { useEffect } from 'react'
import { store } from '../stores/useStore';
import { listen } from '@tauri-apps/api/event';
import { useNavigate } from 'react-router';


export default function Sender() {
  const { serveAndConnectQuic, otp, addConnectedTo, connectedTo } = store()
  
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
    const serveConnect = async () => {
      await serveAndConnectQuic()
    }
    serveConnect()
  }, [])
  
  if (otp == null) {
    return <div>
      loading
    </div>
  } else {
    <div>
      {otp}
    </div>
  }
}