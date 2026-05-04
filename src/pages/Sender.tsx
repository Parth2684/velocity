import { useEffect } from 'react'
import { store } from '../stores/useStore';
import { useNavigate } from 'react-router-dom';


export default function Sender() {
  const { otp, connectedTo, serveAndConnectQuic } = store()
  const nav = useNavigate();
  
  useEffect(() => {
    if (connectedTo !== null) {
      nav("/transfer")
    }
  }, []);
  
  useEffect(() => {
    const serveConnect = async () => {
      await serveAndConnectQuic()
    }
    serveConnect()
  }, [])
  
  if (otp == null) {
    return <div className='m-10 p-10'>
      loading
    </div>
  } else {
    return <div className='m-10 p-10 text-black'>
      {otp}
    </div>
  }
}