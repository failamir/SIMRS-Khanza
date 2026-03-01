import { For } from 'solid-js';

interface MenuItem {
  id: string;
  label: string;
  icon: string;
  children?: MenuItem[];
}

const menuItems: MenuItem[] = [
  { id: 'dashboard', label: 'Dashboard', icon: '📊' },
  { id: 'registrasi', label: 'Registrasi', icon: '📝' },
  {
    id: 'rawat-jalan',
    label: 'Rawat Jalan',
    icon: '🏥',
    children: [
      { id: 'pendaftaran-rj', label: 'Pendaftaran', icon: '📝' },
      { id: 'pemeriksaan-rj', label: 'Pemeriksaan', icon: '🩺' },
      { id: 'reseprj', label: 'Resep', icon: '💊' },
    ]
  },
  {
    id: 'rawat-inap',
    label: 'Rawat Inap',
    icon: '🛏️',
    children: [
      { id: 'pendaftaran-ri', label: 'Pendaftaran', icon: '📝' },
      { id: 'pemeriksaan-ri', label: 'Pemeriksaan', icon: '🩺' },
      { id: 'resepri', label: 'Resep', icon: '💊' },
    ]
  },
  { id: 'farmasi', label: 'Farmasi', icon: '💊' },
  { id: 'laboratorium', label: 'Laboratorium', icon: '🔬' },
  { id: 'radiologi', label: 'Radiologi', icon: '📡' },
  {
    id: 'keuangan',
    label: 'Keuangan',
    icon: '💰',
    children: [
      { id: 'tagihan', label: 'Tagihan', icon: '📄' },
      { id: 'pembayaran', label: 'Pembayaran', icon: '💳' },
      { id: 'piutang', label: 'Piutang', icon: '📈' },
    ]
  },
  {
    id: 'bridging',
    label: 'Bridging',
    icon: '🔗',
    children: [
      { id: 'bpjs', label: 'BPJS', icon: '🏥' },
      { id: 'satu-sehat', label: 'Satu Sehat', icon: '❤️' },
    ]
  },
  { id: 'laporan', label: 'Laporan', icon: '📋' },
  { id: 'master', label: 'Data Master', icon: '📚' },
  { id: 'setting', label: 'Pengaturan', icon: '⚙️' },
];

interface SidebarProps {
  open: boolean;
  currentPage: string;
  onNavigate: (page: string) => void;
}

export default function Sidebar(props: SidebarProps) {
  return (
    <aside class="sidebar" classList={{ collapsed: !props.open }}>
      <div class="sidebar-header">
        <div class="logo">
          <span class="logo-icon">🏥</span>
          {props.open && <span class="logo-text">SIMRS Khanza</span>}
        </div>
      </div>

      <nav class="sidebar-nav">
        <For each={menuItems}>
          {(item) => (
            <div class="menu-item">
              <button
                class={`menu-button ${props.currentPage === item.id ? 'active' : ''}`}
                onClick={() => props.onNavigate(item.id)}
              >
                <span class="menu-icon">{item.icon}</span>
                {props.open && <span class="menu-label">{item.label}</span>}
              </button>
              {item.children && props.open && (
                <div class="submenu">
                  <For each={item.children}>
                    {(child) => (
                      <button
                        class={`submenu-button ${props.currentPage === child.id ? 'active' : ''}`}
                        onClick={() => props.onNavigate(child.id)}
                      >
                        <span class="menu-icon">{child.icon}</span>
                        <span class="menu-label">{child.label}</span>
                      </button>
                    )}
                  </For>
                </div>
              )}
            </div>
          )}
        </For>
      </nav>

      <div class="sidebar-footer">
        <div class="user-info">
          <div class="user-avatar">👤</div>
          {props.open && (
            <div class="user-details">
              <div class="user-name">Admin</div>
              <div class="user-role">Administrator</div>
            </div>
          )}
        </div>
      </div>
    </aside>
  );
}