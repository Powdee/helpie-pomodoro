<script lang="ts">
  import Timer from './lib/timer.svelte';
  import Break from './lib/break.svelte';

  import { isPermissionGranted } from '@tauri-apps/api/notification';
  import { onMount } from 'svelte';

  let permissionGranted = false;
  onMount(async () => {
    permissionGranted = await isPermissionGranted();
  });
</script>

<main>
  <section>
    <h1>permissionGranted: {permissionGranted}</h1>
    <Timer {permissionGranted} />
    <Break {permissionGranted} />
  </section>
</main>
