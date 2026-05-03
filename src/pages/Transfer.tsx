import { useEffect } from 'react';
import { store } from '../stores/useStore';
import { useNavigate } from 'react-router';

export default function Transfer() {
  const { connectedTo, send_files, receive_files } = store()
  const nav = useNavigate()
  useEffect(() => {
    if (connectedTo == null) {
      nav("/")
    }
  }, [connectedTo])
  return <div>
    {Array.from(send_files.entries()).map(([key, file]) => (
      <div id={key}>{ JSON.stringify(file) }</div>
    ))}
    {Array.from(receive_files.entries()).map(([key, file]) => (
      <div id={key}>{ JSON.stringify(file) }</div>
    ))}
  </div>;
}
