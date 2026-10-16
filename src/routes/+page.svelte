<script lang="ts">
  let activeTab = $state<'creator' | 'applicator' | 'sounds' | 'cursors' | 'settings'>('creator');
  let themeName = $state('My Custom Theme');
  let platform = $state<'windows' | 'macos' | 'linux'>('windows');
</script>

<div class="studio-container">
  <header class="studio-header">
    <div class="brand">
      <span class="logo-badge">TT</span>
      <div class="title-group">
        <h1>Theme Toolkit</h1>
        <span class="mode-badge">Local Studio & Applicator</span>
      </div>
    </div>
    <nav class="nav-tabs">
      <button class="tab-btn" class:active={activeTab === 'creator'} onclick={() => activeTab = 'creator'}>Theme Creator</button>
      <button class="tab-btn" class:active={activeTab === 'applicator'} onclick={() => activeTab = 'applicator'}>Theme Applicator</button>
      <button class="tab-btn" class:active={activeTab === 'sounds'} onclick={() => activeTab = 'sounds'}>Sound Schemes</button>
      <button class="tab-btn" class:active={activeTab === 'cursors'} onclick={() => activeTab = 'cursors'}>Cursor Schemes</button>
      <button class="tab-btn" class:active={activeTab === 'settings'} onclick={() => activeTab = 'settings'}>Settings</button>
    </nav>
  </header>

  <main class="studio-main">
    <div class="workspace-card">
      <h2>Welcome to Theme Toolkit</h2>
      <p class="subtitle">
        A 100% local, privacy-respecting desktop theming studio. Create, inspect, package, and apply desktop customizations directly on your machine without online stores or remote catalogues.
      </p>

      <div class="platform-selector">
        <label>Target Platform:</label>
        <div class="pill-group">
          <button class="pill" class:selected={platform === 'windows'} onclick={() => platform = 'windows'}>Windows 10 / 11</button>
          <button class="pill" class:selected={platform === 'macos'} onclick={() => platform = 'macos'}>macOS</button>
          <button class="pill" class:selected={platform === 'linux'} onclick={() => platform = 'linux'}>Linux (FreeDesktop)</button>
        </div>
      </div>

      <div class="features-grid">
        <div class="feature-tile">
          <h3>📦 Standard Packaging</h3>
          <p>Compiles lossless <code>.deskthemepack</code> archives via <code>makecab.exe</code>, macOS dynamic HEIC bundles, and Linux FreeDesktop packages.</p>
        </div>
        <div class="feature-tile">
          <h3>🎵 Sound Scheme Studio</h3>
          <p>Maps system audio events with loudness normalization to -14 LUFS, supporting Win32 event registries and FreeDesktop sound themes.</p>
        </div>
        <div class="feature-tile">
          <h3>🖱️ Cursor Workshop</h3>
          <p>Configures multi-resolution <code>.cur</code>/<code>.ani</code> pointer sets, retina macOS cursors, and XDG <code>xcursorgen</code> binaries.</p>
        </div>
        <div class="feature-tile">
          <h3>🔒 Pure Local Architecture</h3>
          <p>Zero remote storefronts, zero cloud telemetry. Themes are created, stored, and applied strictly on your local machine.</p>
        </div>
      </div>
    </div>
  </main>
</div>

<style>
  .studio-container {
    display: flex;
    flex-direction: column;
    height: 100%;
    background: radial-gradient(circle at 10% 10%, rgba(26, 159, 255, 0.05), transparent 40%),
                radial-gradient(circle at 90% 90%, rgba(102, 192, 244, 0.04), transparent 40%),
                #0e141b;
  }

  .studio-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 16px 24px;
    background: rgba(21, 29, 39, 0.7);
    backdrop-filter: blur(16px);
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .logo-badge {
    background: linear-gradient(135deg, #1a9fff, #66c0f4);
    color: #040a14;
    font-weight: 800;
    font-size: 14px;
    padding: 6px 10px;
    border-radius: 8px;
    letter-spacing: 0.5px;
  }

  .title-group h1 {
    font-size: 16px;
    margin: 0;
    font-weight: 700;
  }

  .mode-badge {
    font-size: 11px;
    color: #66c0f4;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    font-weight: 600;
  }

  .nav-tabs {
    display: flex;
    gap: 8px;
  }

  .tab-btn {
    background: transparent;
    border: 1px solid transparent;
    color: #8b949e;
    padding: 8px 16px;
    border-radius: 8px;
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .tab-btn:hover {
    color: #f0f6fc;
    background: rgba(255, 255, 255, 0.04);
  }

  .tab-btn.active {
    color: #ffffff;
    background: rgba(26, 159, 255, 0.15);
    border-color: rgba(26, 159, 255, 0.3);
  }

  .studio-main {
    flex: 1;
    overflow-y: auto;
    padding: 32px;
    display: flex;
    justify-content: center;
  }

  .workspace-card {
    max-width: 860px;
    width: 100%;
    background: rgba(21, 29, 39, 0.6);
    border: 1px solid rgba(255, 255, 255, 0.08);
    border-radius: 16px;
    padding: 32px;
    box-shadow: 0 12px 32px rgba(0, 0, 0, 0.3);
  }

  .workspace-card h2 {
    margin: 0 0 8px 0;
    font-size: 24px;
  }

  .subtitle {
    color: #8b949e;
    font-size: 14px;
    line-height: 1.6;
    margin: 0 0 24px 0;
  }

  .platform-selector {
    display: flex;
    align-items: center;
    gap: 16px;
    margin-bottom: 28px;
    padding: 16px;
    background: rgba(14, 20, 27, 0.6);
    border-radius: 10px;
    border: 1px solid rgba(255, 255, 255, 0.04);
  }

  .platform-selector label {
    font-size: 13px;
    font-weight: 600;
    color: #8b949e;
  }

  .pill-group {
    display: flex;
    gap: 8px;
  }

  .pill {
    background: rgba(255, 255, 255, 0.04);
    border: 1px solid rgba(255, 255, 255, 0.08);
    color: #8b949e;
    padding: 6px 14px;
    border-radius: 20px;
    font-size: 12px;
    cursor: pointer;
    transition: all 0.15s ease;
  }

  .pill:hover {
    color: #f0f6fc;
  }

  .pill.selected {
    background: #1a9fff;
    border-color: #1a9fff;
    color: #040a14;
    font-weight: 600;
  }

  .features-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .feature-tile {
    background: rgba(14, 20, 27, 0.5);
    border: 1px solid rgba(255, 255, 255, 0.05);
    border-radius: 12px;
    padding: 20px;
  }

  .feature-tile h3 {
    margin: 0 0 8px 0;
    font-size: 15px;
    color: #66c0f4;
  }

  .feature-tile p {
    margin: 0;
    font-size: 13px;
    color: #8b949e;
    line-height: 1.5;
  }

  .feature-tile code {
    background: rgba(255, 255, 255, 0.06);
    padding: 2px 6px;
    border-radius: 4px;
    font-size: 12px;
    color: #f0f6fc;
  }
</style>
