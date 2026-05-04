import { Routes } from 'react-router-dom';
import "./App.css";
import { Route } from 'react-router-dom';
import Home from './pages/Home';
import Sender from './pages/Sender';
import { useEffect } from 'react';
import { setupListeners } from './listners';
import Scan from './pages/Receiver';
import Transfer from './pages/Transfer';

function App() {
  useEffect(() => {
    let cleanup: () => void | undefined;
    
    setupListeners().then((fn) => {
      cleanup = fn
    }).catch(err => console.error("error unlistening: " + err))
    
    return () => {
      cleanup?.();
    }
  }, [])
 
  return (
    
    <Routes>
      <Route path='/' element={<Home />} />
      <Route path='/serve' element={<Sender />} />
      <Route path="/scan" element={<Scan />} />
      <Route path='/transfer' element={ <Transfer /> } />
    </Routes>
  );
}

export default App;
