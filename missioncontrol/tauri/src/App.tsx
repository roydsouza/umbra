import { Routes, Route } from 'react-router-dom';
import { MainLayout } from './components/layout/MainLayout';
import { Dashboard } from './pages/Dashboard';
import { Circuits } from './pages/Circuits';
import { Guardian } from './pages/Guardian';
import { DarkMatter } from './pages/DarkMatter';
import { Services } from './pages/Services';
import { Metrics } from './pages/Metrics';
import { Integrations } from './pages/Integrations';

function App() {
  return (
    <MainLayout>
      <Routes>
        <Route path="/" element={<Dashboard />} />
        <Route path="/circuits" element={<Circuits />} />
        <Route path="/services" element={<Services />} />
        <Route path="/metrics" element={<Metrics />} />
        <Route path="/integrations" element={<Integrations />} />
        <Route path="/guardian" element={<Guardian />} />
        <Route path="/darkmatter" element={<DarkMatter />} />
      </Routes>
    </MainLayout>
  );
}

export default App;
