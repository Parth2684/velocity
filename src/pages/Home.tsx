import { useNavigate } from 'react-router-dom'
import { Button } from '../components/ui/button'
import { store } from '../stores/useStore'


export default function Home() {
  const { connectedTo } = store.getState()
  const nav = useNavigate()
  return <div className='m-10 p-10'>
    {(connectedTo === null) ? <>
      <Button variant="default" onClick={() => nav("/serve")}>Serve Device</Button>
      <Button variant="default" onClick={() => nav("/scan")}>Scan Device</Button>
    </> : <>
      <Button variant="default" onClick={() => nav("/transfer")}>Transfers</Button>
    </> }
    </div>
}