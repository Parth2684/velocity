import { useEffect } from 'react';
import { store } from '../stores/useStore';
import { useNavigate } from 'react-router';

export default function Transfer() {
  const { connectedTo } = store()
  const nav = useNavigate()
  useEffect(() => {
    if (connectedTo == null) {
      nav("/")
    }
  }, [connectedTo])
  return <div>
    {/*{files.forEach(())}*/}
  </div>;
}
