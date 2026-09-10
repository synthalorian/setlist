<script>
  // v0: core engine (parser + transposition) lives in Rust; UI lands later.
  let songTitle = "Setlist";
</script>

<main>
  <h1>🎹 {songTitle}</h1>
  <p class="tagline">Praise team command center — setlists, chord charts, one-tap transposition.</p>
  <div class="card">
    <h2>v0 core engine</h2>
    <ul>
      <li>ChordPro parser (Rust)</li>
      <li>Transposition engine ±N semitones</li>
      <li>CLI: <code>setlist transpose &lt;file&gt; &lt;semitones&gt;</code></li>
    </ul>
  </div>
</main>

<style>
  main {
    max-width: 720px;
    margin: 0 auto;
    padding: 3rem 1.5rem;
  }
  h1 {
    color: var(--accent);
    margin-bottom: 0.25rem;
  }
  .tagline {
    opacity: 0.8;
  }
  .card {
    background: var(--surface);
    border: 1px solid color-mix(in srgb, var(--text) 12%, transparent);
    border-radius: 8px;
    padding: 1rem 1.25rem;
    margin-top: 1.5rem;
  }
  code {
    color: var(--accent);
  }
</style>
