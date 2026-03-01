import { createSignal } from 'solid-js';
import Sidebar from './components/Sidebar';
import Header from './components/Header';
import Dashboard from './pages/Dashboard';

export default function App() {
  const [currentPage, setCurrentPage] = createSignal('dashboard');
  const [sidebarOpen, setSidebarOpen] = createSignal(true);

  return (
    <div class="app-container">
      <Sidebar
        open={sidebarOpen()}
        currentPage={currentPage()}
        onNavigate={setCurrentPage}
      />
      <main class="main-content">
        <Header
          onToggleSidebar={() => setSidebarOpen(!sidebarOpen())}
          pageTitle={currentPage()}
        />
        <div class="page-content">
          <Dashboard />
        </div>
      </main>
    </div>
  );
}