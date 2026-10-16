<script lang="ts">
  import type { Snippet } from 'svelte';

  interface Props {
    children?: Snippet;
  }

  let { children }: Props = $props();
</script>

<div class="theme-toolkit-app">
  {@render children?.()}
</div>

<style>
  :global(:root) {
    --bg-dark: #0e141b;
    --panel-dark: #151d27;
    --accent-blue: #1a9fff;
    --accent-light: #66c0f4;
    --text-primary: #f0f6fc;
    --text-muted: #8b949e;
    --border-glass: rgba(255, 255, 255, 0.08);
  }

  :global(body) {
    margin: 0;
    padding: 0;
    background-color: var(--bg-dark);
    color: var(--text-primary);
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Helvetica, Arial, sans-serif;
    user-select: none;
    overflow: hidden;
  }

  .theme-toolkit-app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    width: 100vw;
  }
</style>
