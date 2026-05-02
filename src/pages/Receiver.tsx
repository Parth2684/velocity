import { useEffect, useState } from 'react';
import { store } from '../stores/useStore';
import { AvailableDevice, Discovery } from '../stores/types';
import { Button } from '../components/ui/button';

export default function Scan() {
  const { scan, availableDevices, receiveCertAndConnectQuic } = store()
  const [device, setDevice] = useState<AvailableDevice>()
  const [showInput, setShowInput] = useState(false);
  const [otp, setOtp] = useState<string>("");
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
      }}>{ device.fullname }</Button> </div>
    ))}
    {showInput ?? <div>
      <input type='text' onChange={(e) => setOtp(e.target.value)}></input>
      <Button type='submit' onClick={async() => {
        receiveCertAndConnectQuic(device!.txt_properties, otp)
      }} />
    </div>}
  </div>
  
  
}