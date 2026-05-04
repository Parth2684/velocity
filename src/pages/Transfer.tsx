import { useEffect } from 'react';
import { store } from '../stores/useStore';
import { useNavigate } from 'react-router-dom';
import { Button } from '../components/ui/button';
import { open } from '@tauri-apps/plugin-dialog';

export default function Transfer() {
  const { connectedTo, send_files, receive_files, send } = store.getState()
  const nav = useNavigate()
  useEffect(() => {
    if (connectedTo == null) {
      nav("/")
    }
  }, [connectedTo])
  return <div>
    <Button onClick={async () => {
      const files = await open({
        multiple: true,
        directory: false
      }) 
      if (files === null || files.length === 0) return
      await send(files)
    }}>Send</Button>
    {Array.from(send_files.entries()).map(([key, file]) => (
      <div id={key}>{ JSON.stringify(file) }</div>
    ))}
    {Array.from(receive_files.entries()).map(([key, file]) => (
      <div id={key}>{ JSON.stringify(file) }</div>
    ))}
  </div>;
}
