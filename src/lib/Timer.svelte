<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { sendNotification } from '@tauri-apps/api/notification';
  import { store_pomodoro } from '../../src/store';

  export let permissionGranted: boolean;

  let current_id: number = 0;

  let start_time = 10;
  let stopped_time = null;
  let pause = true;

  const update_count = () => {
    start_time = start_time - 1;
  };

  let count_interval = null;

  $: if (start_time === 0) {
    clearInterval(count_interval);
    pause = true;

    const current_history_data = $store_pomodoro.get(current_id);
    const stopped_info = current_history_data.stopped || [];

    $store_pomodoro = $store_pomodoro.set(current_id, {
      ...current_history_data,
      stopped: [...stopped_info, 0],
      finished: new Date()
    });

    if (permissionGranted) sendNotification('You are done! Time for a break');
  }

  const start_counting = () => {
    count_interval = setInterval(update_count, 1000);
    pause = false;

    const current_history_data = $store_pomodoro.get(current_id);
    if (current_history_data) {
      $store_pomodoro = $store_pomodoro.set(current_id, {
        ...current_history_data,
        started: [...current_history_data.started, start_time]
      });
    } else {
      current_id = Date.now();
      $store_pomodoro = $store_pomodoro.set(current_id, {
        created: new Date(),
        started: [start_time]
      });
    }

    const invoked_obj = $store_pomodoro.get(current_id);
    invoke('gather_history_data', {
      state: JSON.stringify(invoked_obj)
    });
  };

  const stop_counting = () => {
    pause = true;
    clearInterval(count_interval);
    stopped_time = start_time;

    const current_history_data = $store_pomodoro.get(current_id);
    const stopped_info = current_history_data.stopped || [];

    $store_pomodoro = $store_pomodoro.set(current_id, {
      ...current_history_data,
      stopped: [...stopped_info, stopped_time]
    });
  };

  const reset_counting = () => {
    current_id = null;
    clearInterval(count_interval);
    start_time = 10;
  };

  const convert_seconds_to_hhmmss = (seconds: number) =>
    new Date(seconds * 1000).toISOString().slice(11, 19);
</script>

<button on:click={start_counting} disabled={start_time === 0 || !pause}>
  Start
</button>
<button on:click={stop_counting} disabled={start_time === 0 || pause}>
  Stop
</button>
<button on:click={reset_counting} disabled={!pause}> Reset </button>

<h2>{convert_seconds_to_hhmmss(start_time)}</h2>

<ul>
  {#each [...$store_pomodoro] as [_, data]}
    <li>
      {new Date(data.created).getDate()}.{new Date(
        data.created
      ).getMonth()}.{new Date(data.created).getFullYear()}
      <ul class="started">
        {#each data.started as started}
          <li><span>{convert_seconds_to_hhmmss(started)},</span></li>
        {/each}
      </ul>

      <ul class="started">
        {#if data.stopped}
          {#each data.stopped as stopped}
            <li>{convert_seconds_to_hhmmss(stopped)},</li>
          {/each}
        {/if}
      </ul>
    </li>
  {/each}
</ul>

<style>
  .started {
    display: flex;
    flex-direction: row;
  }

  .started li {
    list-style: none;
  }
</style>
