<script lang="ts">
  import { sendNotification } from '@tauri-apps/api/notification';
  import { convert_seconds_to_hhmmss } from '../core/general/date';

  export let permissionGranted: boolean;

  const START_TIME_DEFAULT_MINUTES = 5;
  const START_TIME_DEFAULT_SECONDS = 0;

  let take_break_minutes = START_TIME_DEFAULT_MINUTES;
  let take_break_seconds = START_TIME_DEFAULT_SECONDS;

  $: start_time = take_break_minutes * 60 + take_break_seconds;
  let stopped_time = null;
  let pause = true;

  const update_count = () => {
    start_time -= 1;
  };

  const on_click_take_break = async () => {
    count_interval = setInterval(update_count, 1000);
    pause = false;
  };

  let count_interval = null;

  $: if (start_time === 0) {
    clearInterval(count_interval);
    pause = true;

    if (permissionGranted) sendNotification('Break is over !');
  }

  const on_click_add_minutes = () => {
    start_time += 60;
  };
</script>

<input
  type="number"
  id="minutes"
  maxlength={2}
  max={59}
  min={1}
  bind:value={take_break_minutes}
/>

<input
  type="number"
  id="seconds"
  maxlength={2}
  max={59}
  min={0}
  bind:value={take_break_seconds}
/>

<button on:click={on_click_take_break} disabled={!pause}> Take a break </button>
<button on:click={on_click_add_minutes}> + 1 min </button>

<h2>{convert_seconds_to_hhmmss(start_time)}</h2>
