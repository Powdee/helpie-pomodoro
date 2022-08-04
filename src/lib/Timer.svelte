<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { sendNotification } from '@tauri-apps/api/notification';
  import { store_pomodoro } from '../core/store';
  import {
    convert_seconds_to_hhmmss,
    convert_date_to_full_readable_date
  } from '../core/general/date';

  export let permissionGranted: boolean;

  let current_id: number = 0;

  const START_TIME_DEFAULT_MINUTES = 25;
  const START_TIME_DEFAULT_SECONDS = 0;

  let start_time_minutes = START_TIME_DEFAULT_MINUTES;
  let start_time_seconds = START_TIME_DEFAULT_SECONDS;

  $: start_time = start_time_minutes * 60 + start_time_seconds;
  let stopped_time = null;
  let pause = true;

  const update_count = () => {
    start_time -= 1;

    if (start_time == 60 && permissionGranted)
      sendNotification('Hang up! Almost there. 1 minute remaining');
  };

  let count_interval = null;

  // once start time is over we need to clear interval and pause the counting.
  // store neccessary data, reset counting and send notification
  // once we are done
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

    create_new_session();
  }

  const start_session = async () => {
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
    await invoke('gather_history_data', {
      state: JSON.stringify(invoked_obj)
    });
  };

  const pause_session = () => {
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

  const create_new_session = () => {
    current_id = null;
    clearInterval(count_interval);
  };

  const remove_session = (id: number) => {
    $store_pomodoro.delete(id);
    $store_pomodoro = $store_pomodoro;

    if (id === current_id) {
      create_new_session();
      pause = true;
    }
  };

  const remove_all_sessions = () => {
    $store_pomodoro.clear();
    $store_pomodoro = $store_pomodoro;
    create_new_session();
  };
</script>

<input
  type="number"
  id="minutes"
  maxlength={2}
  max={59}
  min={1}
  bind:value={start_time_minutes}
/>

<input
  type="number"
  id="seconds"
  maxlength={2}
  max={59}
  min={0}
  bind:value={start_time_seconds}
/>

<button on:click={start_session} disabled={start_time === 0 || !pause}>
  Start Session
</button>
<button on:click={pause_session} disabled={start_time === 0 || pause}>
  Pause Session
</button>
{#if $store_pomodoro.size > 0}
  <button on:click={create_new_session} disabled={!pause}>
    Create New Session
  </button>
  <button on:click={remove_all_sessions} disabled={!pause}> Clear All </button>
{/if}

<h2>{convert_seconds_to_hhmmss(start_time)}</h2>

<ul>
  {#each [...$store_pomodoro] as [id, data]}
    <li>
      {convert_date_to_full_readable_date(data.created)}
      <button on:click={() => remove_session(id)}>Delete</button>
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
