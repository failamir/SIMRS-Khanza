import { createSignal, onMount } from 'solid-js';
import { invoke } from '@tauri-apps/api/core';

interface DashboardStats {
  totalPasien: number;
  pasienHariIni: number;
  rawatJalan: number;
  rawatInap: number;
}

export default function Dashboard() {
  const [stats, setStats] = createSignal<DashboardStats>({
    totalPasien: 0,
    pasienHariIni: 0,
    rawatJalan: 0,
    rawatInap: 0
  });

  const [recentPatients, setRecentPatients] = createSignal<any[]>([]);

  onMount(async () => {
    try {
      // Fetch stats from Rust backend
      const dashboardStats = await invoke<DashboardStats>('get_dashboard_stats');
      setStats(dashboardStats);

      // Fetch recent patients
      const patients = await invoke<any[]>('get_recent_patients');
      setRecentPatients(patients);
    } catch (error) {
      console.error('Failed to fetch dashboard data:', error);
      // Set mock data for development
      setStats({
        totalPasien: 12458,
        pasienHariIni: 42,
        rawatJalan: 38,
        rawatInap: 12
      });
    }
  });

  return (
    <div class="dashboard">
      <div class="stats-grid">
        <div class="stat-card">
          <div class="stat-card-icon" style="background: rgba(37, 99, 235, 0.1); color: #2563eb;">
            👥
          </div>
          <div class="stat-card-value">{stats().totalPasien.toLocaleString()}</div>
          <div class="stat-card-label">Total Pasien</div>
        </div>

        <div class="stat-card">
          <div class="stat-card-icon" style="background: rgba(34, 197, 94, 0.1); color: #22c55e;">
            📅
          </div>
          <div class="stat-card-value">{stats().pasienHariIni}</div>
          <div class="stat-card-label">Pasien Hari Ini</div>
        </div>

        <div class="stat-card">
          <div class="stat-card-icon" style="background: rgba(245, 158, 11, 0.1); color: #f59e0b;">
            🏥
          </div>
          <div class="stat-card-value">{stats().rawatJalan}</div>
          <div class="stat-card-label">Rawat Jalan</div>
        </div>

        <div class="stat-card">
          <div class="stat-card-icon" style="background: rgba(239, 68, 68, 0.1); color: #ef4444;">
            🛏️
          </div>
          <div class="stat-card-value">{stats().rawatInap}</div>
          <div class="stat-card-label">Rawat Inap</div>
        </div>
      </div>

      <div class="dashboard-grid">
        <div class="card">
          <div class="card-header">
            <h3 class="card-title">Pasien Terbaru</h3>
            <button class="btn btn-primary">Lihat Semua</button>
          </div>
          <div class="table-container">
            <table>
              <thead>
                <tr>
                  <th>No. RM</th>
                  <th>Nama Pasien</th>
                  <th>Tanggal Daftar</th>
                  <th>Poliklinik</th>
                  <th>Status</th>
                </tr>
              </thead>
              <tbody>
                {recentPatients().length > 0 ? (
                  recentPatients().map((patient) => (
                    <tr>
                      <td>{patient.noRm}</td>
                      <td>{patient.nama}</td>
                      <td>{patient.tanggal}</td>
                      <td>{patient.poli}</td>
                      <td>
                        <span class={`status-badge ${patient.status}`}>
                          {patient.status}
                        </span>
                      </td>
                    </tr>
                  ))
                ) : (
                  <tr>
                    <td colspan="5" style="text-align: center; padding: 40px;">
                      Belum ada data pasien
                    </td>
                  </tr>
                )}
              </tbody>
            </table>
          </div>
        </div>

        <div class="card">
          <div class="card-header">
            <h3 class="card-title">Antrian Hari Ini</h3>
          </div>
          <div class="queue-list">
            <div class="queue-item">
              <span class="queue-number">A001</span>
              <span class="queue-name">Ahmad Suryadi</span>
              <span class="queue-status waiting">Menunggu</span>
            </div>
            <div class="queue-item">
              <span class="queue-number">A002</span>
              <span class="queue-name">Siti Nurhaliza</span>
              <span class="queue-status called">Dipanggil</span>
            </div>
            <div class="queue-item">
              <span class="queue-number">A003</span>
              <span class="queue-name">Budi Santoso</span>
              <span class="queue-status waiting">Menunggu</span>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}