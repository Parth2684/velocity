import { useNavigate } from 'react-router'
import { Button } from '../components/ui/button'
import { store } from '../stores/useStore'


export default function Home() {
  const { connectedTo } = store()
  const nav = useNavigate()
  return <div>
    {!connectedTo ? <>
      <Button onClick={() => nav("/serve")}>Serve Device</Button>
      <Button onClick={() => nav("/scan")}>Scan Device</Button>
    </> : <>
      <Button onClick={() => nav("/Transfers")}>Show Transfers</Button>
    </> }
    </div>
}