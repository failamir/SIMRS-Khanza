interface HeaderProps {
  onToggleSidebar: () => void;
  pageTitle: string;
}

export default function Header(props: HeaderProps) {
  return (
    <header class="header">
      <div class="header-left">
        <button class="toggle-btn" onClick={props.onToggleSidebar}>
          ☰
        </button>
        <h1 class="page-title">{props.pageTitle}</h1>
      </div>

      <div class="header-right">
        <div class="header-search">
          <input type="text" placeholder="Cari pasien, nomor RM..." class="search-input" />
          <button class="search-btn">🔍</button>
        </div>

        <div class="header-actions">
          <button class="header-btn notification-btn">
            🔔
            <span class="notification-badge">3</span>
          </button>
          <button class="header-btn">⚙️</button>
        </div>
      </div>
    </header>
  );
}