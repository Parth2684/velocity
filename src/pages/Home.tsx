import { useNavigate } from 'react-router'
import { Button } from '../components/ui/button'


export default function Home() {
  const nav = useNavigate()
  return <div>
    <Button onClick={() => nav("/sender")}>Send</Button>
    <Button onClick={() => nav("/receiver")}>Receive</Button>
  </div>
}