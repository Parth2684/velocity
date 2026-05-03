import { Routes } from 'react-router';
import "./App.css";
import { Route } from 'react-router';
import Home from './pages/Home';
import Sender from './pages/Sender';
import { useEffect } from 'react';
import { setupListeners } from './listners';
import Scan from './pages/Receiver';

function App() {
  useEffect(() => {
    let cleanup: () => void;
    
    setupListeners().then((fn) => {
      cleanup = fn
    })
    
    return () => {
      if (cleanup) cleanup()
    }
  }, [])

  return (
    
    <Routes>
      <Route path="/" element={<Home />} />
      <Route path='/serve' element={<Sender />} />
      <Route path="/scan" element={<Scan />} />
    </Routes>
  );
}

export default App;
