import { useEffect } from 'react'
import { store } from '../stores/useStore';
import { useNavigate } from 'react-router-dom';


export default function Sender() {
  const { serveAndConnectQuic, otp, connectedTo } = store()
  
  const nav = useNavigate();
  
  useEffect(() => {
    if (connectedTo !== null) {
      nav("/transfer")
    }
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