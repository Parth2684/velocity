import { useEffect } from 'react'
import { store } from '../stores/useStore';


export default function Sender() {
  const { serveAndConnectQuic, otp } = store()
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